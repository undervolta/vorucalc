// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{
	error::Error, path::PathBuf
};
use regex::Regex;
use slint::{ Image, Model, SharedString, StyledText };

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
	
	let state_handle = state.as_weak();
	logic.on_parse_calc(move || {
		if let Some(app_state) = state_handle.upgrade() {
			let res = match lang::calc::parse_calc(app_state.get_calc_input(), app_state.get_var()) {
				Ok(res) => {
					if !res.contains("?") {
						app_state.set_error_msg(SharedString::new());
					} else {
						let lines = res.split("\n").collect::<Vec<&str>>();
						let lines_len = lines.len() - 1;

						let err_lines = lines.into_iter().enumerate().fold(String::new(), |mut acc, (idx, line)| {
							if line.contains("?") {
								if idx < lines_len - 1 { 
									acc.push_str(&format!("{}, ", &(idx + 1).to_string())[..]);
								}
								else {
									acc.push_str(&(idx + 1).to_string());
								}
							}

							acc
						});

						app_state.set_error_msg(SharedString::from(format!("Unknown variable(s) at {} {}.", 
							if err_lines.contains(",") { "lines" } else { "line" },
							err_lines
						)));
					}

					res
				}, 
				Err(err) => {
					let re = Regex::new(r"\d+\.\.\d+").unwrap();
					
					if let Some(mat) = re.find(&err.to_string()) {
						if let &[first, .., last] = &mat.as_str().split("..").collect::<Vec<&str>>()[..] {
							let first_pos = first.parse::<usize>().unwrap_or(0);
							let last_pos = last.parse::<usize>().unwrap_or(0);
							let is_same_pos = first == last;

							let is_calc_input = err.starts_with("calc");
							let input_err = if is_calc_input { app_state.get_calc_input() } else { app_state.get_var() }
								.replace(" ", "").replace("\t", "");
							
							let get_line = | search_pos: usize | -> i32 {
								let mut line = 1;

								for (pos, input) in input_err.chars().enumerate() {
									if input == '\n' { line += 1; }
									if pos == search_pos { return line; }
								}

								line
							};

							let first_line = get_line(first_pos);
							let mut last_line = first_line;

							if !is_same_pos {
								last_line = get_line(last_pos);
							}
							
							app_state.set_error_msg(SharedString::from(format!("{} error at {} {}.", 
								if is_calc_input { "Calculation" } else { "Variable" },
								if is_same_pos { "line" } else { "lines" },
								if is_same_pos { first_line.to_string() } else { format!("{} - {}", first_line, last_line) }
							)));
						};
					}

					SharedString::from("?")
				}
			};

			app_state.set_calc_res(res);
		}
	});

	app.run()?;

	Ok(())
}
