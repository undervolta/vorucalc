// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{
	error::Error,
	path::{ PathBuf }
};
use slint::{ Image, Model, StyledText };

slint::include_modules!();

mod components;
mod lang;

fn main() -> Result<(), Box<dyn Error>> {
	let app = AppWindow::new()?;
	let state = app.global::<AppState>();
	let logic = app.global::<AppLogic>();

	let tabs = components::tabs::get_app_tabs();

	app.set_tabs(tabs.clone());


	let app_handle = app.as_weak();
	logic.on_load_image(move |path| {
		if let Some(_) = app_handle.upgrade() {
			let manifest_dir = PathBuf::from(std::env::var_os("CARGO_MANIFEST_DIR").unwrap());
			let img_path = &manifest_dir.join("ui/icons").join(path.as_str());

			Image::load_from_path(img_path)
				.unwrap_or_else(|_| Image::default())
		}
		else {
			Image::default()
		}
	});

	let app_handle = app.as_weak();
	logic.on_select_tab(move |id, selected| {
		if let Some(ui) = app_handle.upgrade() {
			if let Some((idx, mut tab_group)) = tabs.iter().enumerate().find(|(_, tg)| tg.id == id) {
				tab_group.selected = selected + 1;
				ui.set_active_tabs(tab_group.tabs.row_data(selected as usize).unwrap().name);
				tabs.set_row_data(idx, tab_group);
			}
		}
	});

	let app_handle = app.as_weak();
	logic.on_convert_to_md(move |content| {
		match app_handle.upgrade() {
			Some(_) => StyledText::from_plain_text(&content.to_owned()),
			None => StyledText::default()
		}
	});
	
	let app_handle = app.as_weak();
	let state_handle = state.as_weak();
	logic.on_parse_calc(move |input| {
		if let Some(_) = app_handle.upgrade() {
			if let Some(app_state) = state_handle.upgrade() {
				app_state.set_calc_res(lang::parser::parse_calc(input));
			}
		}
	});

	app.run()?;

	Ok(())
}
