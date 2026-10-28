use std::rc::Rc;

use slint::{ VecModel, ModelRc };
use crate::slint_generatedAppWindow::{ TabsState, TabsType };

pub fn get_app_tabs() -> ModelRc<TabsState> {
	ModelRc::from(Rc::new(VecModel::from(vec![
		TabsState {
			id: "main-nav".into(),
			selected: 1,
			tabs: ModelRc::from(Rc::new(VecModel::from(vec![
				TabsType {
					name: "calc".into(),
					label: "Calculate".into(),
					icon: "calc.svg".into(),
					is_active: true,
					width: 0.0
				},
				TabsType {
					name: "var".into(),
					label: "Variables".into(),
					icon: "var.svg".into(),
					is_active: false,
					width: 0.0
				},
				TabsType {
					name: "note".into(),
					label: "Notes".into(),
					icon: "notebook.svg".into(),
					is_active: false,
					width: 0.0
				},
				// TabsType {
				// 	name: "stg".into(),
				// 	label: "".into(),
				// 	icon: "cog.svg".into(),
				// 	is_active: false,
				// 	width: 14.0
				// },
			])))
		}
	])))
}
