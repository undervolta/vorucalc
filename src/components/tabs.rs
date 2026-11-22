use std::rc::Rc;

use slint::{ VecModel, ModelRc };
use crate::{ AppState, slint_generatedAppWindow::{ TabsState, TabsType } };

pub fn get_app_tabs(state: &AppState<'_>) -> ModelRc<TabsState> {
	let user_settings = state.get_user_settings();
	let show_var = user_settings.separate_var.2;
	let show_notes = user_settings.show_notes.2;

	let mut tabs = vec![
		TabsType {
			name: "calc".into(),
			label: "Calculate".into(),
			icon: "calc".into(),
			is_active: true,
			width: 0.0
		},
	];

	if show_var {
		tabs.push(TabsType {
			name: "var".into(),
			label: "Variables".into(),
			icon: "var".into(),
			is_active: false,
			width: 0.0
		});
	}
	
	if show_notes {
		tabs.push(TabsType {
			name: "note".into(),
			label: "Notes".into(),
			icon: "notebook".into(),
			is_active: false,
			width: 0.0
		});
	}

	ModelRc::from(Rc::new(VecModel::from(vec![
		TabsState {
			id: "main-nav".into(),
			selected: 1,
			tabs: ModelRc::from(Rc::new(VecModel::from(tabs)))
		}
	])))
}
