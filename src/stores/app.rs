use std::{ fmt, path::PathBuf };
use slint::{ 
	Model, ModelRc,
	language::ColorScheme
};

use crate::{ 
	AppLogic, AppState, Palette, SettingsOption, Theme, slint_generatedAppWindow::{ SemanticSize, UserSettingType, UserSettings }, stores::storage::UserSettings as SavedUserSettings
};

pub struct SaveSettingsHelper<'a, 'b> {
	pub state: &'a AppState<'a>,
	pub logic: Option<&'a AppLogic<'a>>,
	pub palette: &'a Palette<'a>,
	pub theme: &'a Theme<'a>,
	pub path: &'b PathBuf
}

impl UserSettings {
	pub fn new() -> UserSettings {
		UserSettings {
			dark_mode: ( "Dark Mode".into(), true ),
			show_err_msg: ( "Show Error Message".into(), true ),
			show_notes: ( "Include the \"Notes\" section in the menu tab".into(), "Show Notes".into(), false ),
			separate_var: ( "Move the list of variables to a dedicated \"Variables\" section in the menu tab".into(), "Separate Variables".into(), true ),
			save_on_close: ( "Save on Close".into(), false ),
			decimal_prec: ( "The maximum number of digits displayed after the decimal separator".into(), "Decimal Precision".into(), "3".into() ),
			decimal_separator: ( "The symbol used to split the whole number part from the fractional part (output only)".into(), "Decimal Separator".into(), ".".into() ),
			digit_separator: ( "The symbol used to group digits in large numbers (output only)".into(), "Digit Separator".into(), ",".into() ),
			font_size: ( "Font Size".into(), SemanticSize::Lg ),
		}
	}
}

impl fmt::Display for UserSettings {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let show = vec![
			format!("dark_mode: {}", self.dark_mode.1),
			format!("show_err_msg: {}", self.show_err_msg.1),
			format!("save_on_close: {}", self.save_on_close.1),
			format!("decimal_prec: {}", self.decimal_prec.2),
			format!("decimal_separator: {}", self.decimal_separator.2),
			format!("digit_separator: {}", self.digit_separator.2),
			format!("font_size: {}", match self.font_size.1 {
				SemanticSize::Xs => "xs",
				SemanticSize::Sm => "sm",
				SemanticSize::Md => "base",
				SemanticSize::Lg => "lg",
				SemanticSize::Xl => "xl",
				SemanticSize::Xxl => "xxl",
				_ => ""
			})
		].join("\n");
		let res = show.as_str();

		write!(f, "{res}")
	}
}

pub fn get_user_settings() -> UserSettings {
	UserSettings::new()
}

pub fn save_user_settings(settings: &ModelRc<SettingsOption>, helper: SaveSettingsHelper) {
	let SaveSettingsHelper { 
		state, 
		logic,
		palette, 
		theme ,
		path
	} = helper;

	let saved_settings = settings.iter()
		.fold(UserSettings::new(), |mut acc, stg| {
			match stg.r#type {
				UserSettingType::DarkMode => {
					state.set_selected_theme(if stg.checked { 1 } else { 0 });
					state.set_palette(theme.get_list().row_data(state.get_selected_theme() as usize)
						.unwrap_or(theme.get_list().row_data(1).unwrap()));

					let dark_theme = state.get_palette().is_dark;

					palette.set_color_scheme(if dark_theme { ColorScheme::Dark } else { ColorScheme::Light });
					acc.dark_mode.1 = dark_theme;
				},
				UserSettingType::DecimalPrecision => {
					acc.decimal_prec.2 = stg.text;
				},
				UserSettingType::DecimalSeparator => {
					acc.decimal_separator.2 = stg.text;
				},
				UserSettingType::DigitSeparator => {
					acc.digit_separator.2 = stg.text;
				},
				UserSettingType::FontSize => {
					acc.font_size.1 = stg.size;
				},
				UserSettingType::SaveOnClose => {
					acc.save_on_close.1 = stg.checked;
				},
				UserSettingType::SeparateVariable => {
					acc.separate_var.2 = stg.checked;
				},
				UserSettingType::ShowErrMessage => {
					acc.show_err_msg.1 = stg.checked;
				},
				UserSettingType::ShowNotes => {
					acc.show_notes.2 = stg.checked;
				}
			};

			acc
		});

	let local_save = SavedUserSettings::new(&saved_settings);

	if let Err(err) = local_save.save_settings(path) {
		eprintln!("Error saving settings: {err}");
	}

	state.set_user_settings(saved_settings);

	if let Some(app_logic) = logic {
		app_logic.invoke_refresh_tab("main-nav".into());
	}
}
