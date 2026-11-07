use crate::slint_generatedAppWindow::{ UserSettings, SemanticSize };

pub fn get_user_settings() -> UserSettings {
	UserSettings {
		dark_mode: ( "Dark Mode".into(), true ),
		show_err_msg: ( "Show Error Message".into(), true ),
		save_on_close: ( "Save on Close".into(), false ),
		decimal_prec: ( "Decimal Precision".into(), "The maximum number of digits that display after the decimal separator".into(), 3 ),
		decimal_separator: ( "Decimal Separator".into(), "The symbol used to split the whole number part from the fractional part".into(), ".".into() ),
		digit_separator: ( "Digit Separator".into(), "The symbol used to group digits in large numbers to make them easier to read".into(), ",".into() ),
		font_size: ( "Font Size".into(), SemanticSize::Lg ),
	}
}
