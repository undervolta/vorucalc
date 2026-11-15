use chumsky::{
	prelude::*
};
use std::collections::HashMap;
use num_format::{ Locale, ToFormattedString };
use slint::{ SharedString };

use crate::lang::{
	common::{ Token, Expr, Value, parse_token, lex, eval, format_float },
	var::parse_var
};

pub struct CalcParsingOptions<'a> {
	pub decimal_prec: i32,
	pub decimal_sep: &'a str,
	pub digit_sep: &'a str
}

fn parser<'a>() -> impl Parser<'a, &'a [Token], Vec<Expr>, extra::Err<Rich<'a, Token>>> {
	let expr = parse_token();

	// statements split newline (any amount of them)
	let sep = just(Token::NewLine)
		.repeated()
		.at_least(1);

	expr.separated_by(sep)
		.allow_leading()
		.allow_trailing()
		.collect::<Vec<_>>()
		.then_ignore(end())
}

pub fn parse_calc(calc_input: SharedString, var_input: SharedString, options: CalcParsingOptions) -> Result<SharedString, SharedString> {
	let tokens = lex(&calc_input);
	let stmts = match parser().parse(tokens.as_slice()).into_result() {
		Ok(s) => s,
		Err(errs) => {
			let err_len = errs.iter().len();
			return Err(SharedString::from(errs.into_iter().enumerate()
				.map(|(idx, err)| format!("calc:{:?}{}", err, if idx < err_len - 1 { "\n" } else { "" }))
				.collect::<String>())
			)
		}
	};

	let CalcParsingOptions {
		decimal_prec,
		decimal_sep,
		digit_sep
	} = options;

	let mut env = HashMap::<i64, Value>::new();
	let vars = parse_var(&var_input)?;

	for (idx, expr) in stmts.iter().enumerate() {
		match eval(expr, &vars) {
			Some(val) => {
				env.insert((idx + 1) as i64, val);
			},
			None => {
				env.insert((idx + 1) as i64, Value::Invalid);
			}
		}
	}

	let env_iter = env.iter().enumerate();
	let env_len = env_iter.len();

	let res: String = env_iter
		.map(|(idx, _)| format!("{}{}", 
			match *env.get(&((idx + 1) as i64)).unwrap_or(&Value::Invalid) {
				Value::Float(val) => format_float(val, decimal_prec as usize),
				Value::Int(val) => val.to_formatted_string(&Locale::en).into(),
				Value::Invalid => "?".into()
			}.replace(".", decimal_sep).replace(",", digit_sep), 
			if idx < env_len - 1 { "\n" } else { "" }))
		.collect();
	
	Ok(SharedString::from(res))
}
