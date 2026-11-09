use std::fmt;
use crate::slint_generatedAppWindow::{ UserSettings, SemanticSize };

impl UserSettings {
	pub fn new() -> UserSettings {
		UserSettings {
			dark_mode: ( "Dark Mode".into(), true ),
			show_err_msg: ( "Show Error Message".into(), true ),
			save_on_close: ( "Save on Close".into(), false ),
			decimal_prec: ( "The maximum number of digits that display after the decimal separator".into(), "Decimal Precision".into(), "3".into() ),
			decimal_separator: ( "The symbol used to split the whole number part from the fractional part".into(), "Decimal Separator".into(), ".".into() ),
			digit_separator: ( "The symbol used to group digits in large numbers to make them easier to read".into(), "Digit Separator".into(), ",".into() ),
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

pub fn save_user_settings(settings: UserSettings) {
	
	println!("settings:\n{settings}")
}
