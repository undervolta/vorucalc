use std::rc::Rc;
use slint::{ ModelRc, VecModel };
use crate::{ 
	AppData, 
	slint_generatedAppWindow::{ HelpPair, StringPair }
};


fn shortcut_item(key: &str, value: &str) -> StringPair {
	StringPair { key: key.into(), value: value.into() }
}

fn help_header(label: &str) -> HelpPair {
	HelpPair { label: label.into(), is_header: true, ..Default::default() }
}

fn help_item(label: &str, value: &str, example: &str) -> HelpPair {
	HelpPair { label: label.into(), value: value.into(), example: example.into(), ..Default::default() }
}

pub fn set_shortcut_list(data: &AppData<'_>) {
	data.set_shortcut_list(ModelRc::from(Rc::new(VecModel::from(vec![
		shortcut_item("Switch to Tab 1", "Ctrl+1"),
		shortcut_item("Switch to Tab 2", "Ctrl+2"),
		shortcut_item("Switch to Tab 3", "Ctrl+3"),
		shortcut_item("Open Settings", "Ctrl+,"),
		shortcut_item("Open Shortcut", "Ctrl+/"),
		// shortcut_item("Save Document", "Ctrl + S"),
		// shortcut_item("Save Document As...", "Ctrl + Shift + S"),
		// shortcut_item("Open Document", "Ctrl + O"),
		shortcut_item("Open Help Window", "F1"),
		shortcut_item("Toggle Menu", "F10"),
		shortcut_item("Toggle Fullscreen", "F11"),
	]))));
}

pub fn set_help_list(data: &AppData<'_>) {
	data.set_help_list(ModelRc::from(Rc::new(VecModel::from(vec![
		help_header("Syntax"),
		help_item("Lines", "Each line is one expression, results appear line by line", "1 + 1\n2 * 3"),
		help_item("Variables", "Defined in the variable pane as name = expr, separated by newline or \";\"", "x = 5; y = x * 2\nz = -x + max(y, 7)"),
		help_item(".func", "Method call, the left side becomes the first argument", "(-4).abs -> 4 \n2.pow(3) -> 8"),

		help_header("Operator"),
		help_item("+", "Addition", "2 + 2 -> 4"),
		help_item("-", "Subtraction / negation", "5 - 3 -> 2 \n-4"),
		help_item("*", "Multiplication", "3 * 4 -> 12"),
		help_item("/", "Division", "7 / 2 -> 3.5"),
		help_item("// or \\", "Integer (floor) division", "7 // 2 -> 3"),
		help_item("%", "Modulo", "7 % 3 -> 1"),
		help_item("^ or **", "Exponent", "2 ^ 10 -> 1,024"),
		help_item("#", "Square root (prefix)", "#16 -> 4"),
		help_item("( )", "Grouping", "(1 + 2) * 3 -> 9"),
		help_item("_", "Digit separator in numbers", "1_000_000"),

		help_header("Constant"),
		help_item("pi", "Pi", "pi -> 3.14159..."),
		help_item("tau", "2 * Pi", "tau -> 6.28318..."),
		help_item("phi", "Golden ratio", "phi -> 1.61803..."),
		help_item("e", "Euler's number", "e -> 2.71828..."),
		help_item("sqrt_2", "Square root of 2", "sqrt_2 -> 1.41421..."),
		help_item("ln2, ln10", "Natural log of 2 / 10", "ln2 -> 0.69314..."),
		help_item("log2_e, log10_e", "Log2 / Log10 of e", "log2_e -> 1.44269..."),
		help_item("log2_10, log10_2", "Log2 of 10 / Log10 of 2", "log10_2 -> 0.30103..."),

		help_header("Rounding"),
		help_item("ceil(x)", "Round up", "ceil(1.2) -> 2"),
		help_item("floor(x)", "Round down", "floor(1.8) -> 1"),
		help_item("round(x)", "Round to nearest", "round(1.5) -> 2"),
		help_item("trunc(x)", "Drop the fractional part", "trunc(-1.7) -> -1"),
		help_item("fract(x)", "Fractional part", "fract(1.25) -> 0.25"),

		help_header("Function"),
		help_item("abs(x)", "Absolute value", "abs(-3) -> 3"),
		help_item("sqrt(x)", "Square root", "sqrt(9) -> 3"),
		help_item("cbrt(x)", "Cube root", "cbrt(27) -> 3"),
		help_item("pow(x, y)", "x raised to the power y", "pow(2, 3) -> 8"),
		help_item("exp(x)", "e raised to x", "exp(1) -> 2.71828..."),
		help_item("exp2(x)", "2 raised to x", "exp2(3) -> 8"),
		help_item("ln(x)", "Natural logarithm", "ln(e) -> 1"),
		help_item("log(x, base)", "Logarithm with given base", "log(8, 2) -> 3"),
		help_item("log10(x)", "Base 10 logarithm", "log10(1000) -> 3"),
		help_item("log2(x)", "Base 2 logarithm", "log2(8) -> 3"),
		help_item("max(x, y)", "Larger of two values", "max(2, 5) -> 5"),
		help_item("min(x, y)", "Smaller of two values", "min(2, 5) -> 2"),
		help_item("sign(x)", "Sign of a number (-1, 0 or 1)", "sign(-8) -> -1"),

		help_header("Trigonometry"),
		help_item("sin(x), cos(x), tan(x)", "Trigonometric functions (radians)", "sin(pi / 2) -> 1"),
		help_item("asin(x), acos(x), atan(x)", "Inverse trigonometric functions", "atan(1) -> 0.78539..."),
		help_item("atan2(y, x)", "Angle of the point (x, y)", "atan2(1, 1) -> 0.78539..."),
		help_item("sinh(x), cosh(x), tanh(x)", "Hyperbolic functions", "sinh(1) -> 1.17520..."),
		help_item("asinh(x), acosh(x), atanh(x)", "Inverse hyperbolic functions", "acosh(1) -> 0"),
		help_item("deg(x)", "Radians to degrees", "deg(pi) -> 180"),
		help_item("rad(x)", "Degrees to radians", "rad(180) -> 3.14159..."),
	]))));
}
