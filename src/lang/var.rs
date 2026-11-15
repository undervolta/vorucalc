use chumsky::{
	prelude::*
};
use std::collections::HashMap;
use slint::{ SharedString };

use crate::lang::common::{ 
	Token, Value, VarStmt,
	parse_token, lex, eval
};


fn parser<'a>() -> impl Parser<'a, &'a [Token], Vec<VarStmt>, extra::Err<Rich<'a, Token>>> {
	let expr = parse_token();
	let ident = select! { Token::Identifier(s) => s };
	let stmt = ident.then_ignore(just(Token::Equal)).then(expr);

	// statements split by `;` or newline (any amount of them)
	let sep = choice((just(Token::Semicolon), just(Token::NewLine)))
		.repeated()
		.at_least(1);

	stmt.separated_by(sep)
		.allow_leading()
		.allow_trailing()
		.collect::<Vec<_>>()
		.then_ignore(end())
}

pub fn parse_var(src: &SharedString) -> Result<HashMap<String, Value>, String> {
	let tokens = lex(src);
	let stmts = match parser().parse(tokens.as_slice()).into_result() {
		Ok(s) => s,
		Err(errs) => return Err(errs.into_iter().map(|err| format!("var:{err:?}\n")).collect()),
	};

	let mut env = HashMap::new();
	for (name, expr) in &stmts {
		match eval(expr, &env) {
			Some(val) => {
				env.insert(name.clone(), val);
			},
			None => {
				env.insert(name.clone(), Value::Invalid);
			}
		}
	}

	Ok(env)
}
