use std::{
	fs, io, path::PathBuf,
	error::Error
};
use serde::{ Serialize, Deserialize };
use slint::{ ComponentHandle, PhysicalPosition, PhysicalSize };
use crate::{ AppState, AppWindow, SemanticSize, UserSettings as FeUserSettings, utils };

pub struct AppStorage {
	pub data: PathBuf,
	pub config: PathBuf,
	// pub cache: PathBuf,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct AppSettings {
	maximize: bool,
	position: (i32, i32),		// (x, y)
    resolution: (u32, u32),		// (w, h)
	saved_input: DocSave,
	settings: UserSettings
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserSettings {
	dark_mode: bool,
	show_err_msg: bool,
	save_on_close: bool,
	decimal_prec: i32,
	decimal_separator: String,
	digit_separator: String,
	#[serde(with = "utils::fs::size_serde")]
	font_size: SemanticSize,
}

#[derive(Debug, Default, Serialize, Deserialize)]
struct DocSave {
	name: String,
	calc: String,
	var: String,
	note: String
}

impl AppSettings {
	pub fn init(state: &AppState<'_>) -> Self {
		let user_settings = state.get_user_settings();

		AppSettings {
			settings: UserSettings {
				dark_mode: user_settings.dark_mode.1, 
				show_err_msg: user_settings.show_err_msg.1, 
				save_on_close: user_settings.save_on_close.1, 
				decimal_prec: user_settings.decimal_prec.2.as_str().parse::<i32>().unwrap_or(3), 
				decimal_separator: user_settings.decimal_separator.2.to_string(), 
				digit_separator: user_settings.digit_separator.2.to_string(), 
				font_size: user_settings.font_size.1,
			},
			..Default::default()
		}
	}
}

impl AppStorage {
	pub fn new(app_name: &str) -> io::Result<Self> {
		let data = sysdirs::data_dir()
			.ok_or_else(|| io::Error::new(
				io::ErrorKind::NotFound,
				"Data directory unavailable",
			))?
			.join(app_name);

		let config = sysdirs::config_dir()
			.ok_or_else(|| io::Error::new(
				io::ErrorKind::NotFound,
				"Config directory unavailable",
			))?
			.join(app_name);

		// let cache = sysdirs::cache_dir()
		// 	.ok_or_else(|| io::Error::new(
		// 		io::ErrorKind::NotFound,
		// 		"Cache directory unavailable",
		// 	))?
		// 	.join(app_name);

		std::fs::create_dir_all(&data)?;
		std::fs::create_dir_all(&config)?;
		// std::fs::create_dir_all(&cache)?;

		Ok(Self {
			data,
			config,
			// cache,
		})
	}

	pub fn save_settings(&self, app: &AppWindow, state: &AppState<'_>) -> Result<(), Box<dyn Error>> {
		let app_window = app.window();
		let PhysicalPosition { x, y } = app_window.position();
		let PhysicalSize { height, width } = app_window.size();
		let FeUserSettings { save_on_close, .. } = state.get_user_settings();
		
		let file_path = self.config.join("settings.toml");
		let mut data: AppSettings = match fs::read_to_string(&file_path) {
			Ok(content) if content.trim().is_empty() => AppSettings::init(state),
			Ok(content) => toml::from_str(&content)?,
			Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
				AppSettings::init(state)
			}
			Err(e) => return Err(e.into()),
		};

		data.maximize = app_window.is_maximized();
		data.position = (x, y);
		data.resolution = (width, height);
		data.saved_input = if save_on_close.1 {
			DocSave { 
				name: "default".into(), 
				calc: state.get_calc_input().to_string(), 
				var: state.get_var().to_string(), 
				note: state.get_note().to_string() 
			}
		} else {
			DocSave { 
				name: "default".into(), 
				..Default::default() }
		};

		let data = toml::to_string(&data)?;
		fs::write(self.config.join("settings.toml"), data)?;
		
		Ok(())
	}
}

impl UserSettings {
	pub fn new(settings: &FeUserSettings) -> Self {
		UserSettings {
			dark_mode: settings.dark_mode.1, 
			show_err_msg: settings.show_err_msg.1, 
			save_on_close: settings.save_on_close.1, 
			decimal_prec: settings.decimal_prec.2.as_str().parse::<i32>().unwrap_or(3), 
			decimal_separator: settings.decimal_separator.2.to_string(), 
			digit_separator: settings.digit_separator.2.to_string(), 
			font_size: settings.font_size.1 
		}
	}

	pub fn save_settings(&self, path: &PathBuf, state: &AppState<'_>) -> Result<(), Box<dyn Error>> {
		let mut data: AppSettings = match fs::read_to_string(path) {
			Ok(content) if content.trim().is_empty() => AppSettings::init(state),
			Ok(content) => toml::from_str(&content)?,
			Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
				AppSettings::init(state)
			}
			Err(e) => return Err(e.into()),
		};

		data.settings = UserSettings {
			dark_mode: self.dark_mode,
			decimal_prec: self.decimal_prec,
			decimal_separator: self.decimal_separator.clone(),
			digit_separator: self.digit_separator.clone(),
			font_size: self.font_size,
			save_on_close: self.save_on_close,
			show_err_msg: self.show_err_msg
		};

		let data = toml::to_string(&data)?;
		fs::write(path, data)?;
		
		Ok(())
	}
}
