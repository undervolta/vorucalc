use chumsky::{
	prelude::*
};
use std::collections::HashMap;
use slint::{ SharedString };

use crate::lang::{
	common::{ Token, BinOp, Expr, Value, lex, eval },
	var::parse_var
};

fn parser<'a>() -> impl Parser<'a, &'a [Token], Vec<Expr>, extra::Err<Rich<'a, Token>>> {
	let expr = recursive(|expr| {
		let atom = select! {
			Token::Number(n) => Expr::Int(n),
			Token::Float(f) => Expr::Float(f),
			Token::Identifier(s) => Expr::Var(s),
			Token::Error => Expr::Invalid,
		}
		.or(expr.delimited_by(just(Token::LParen), just(Token::RParen)));

		let unary = just(Token::Minus)
			.repeated()
			.foldr(atom, |_, rhs| Expr::Neg(Box::new(rhs)));

		let product = unary.clone().foldl(
			choice((
				just(Token::Star).to(BinOp::Mul),
				just(Token::Slash).to(BinOp::Div),
			))
			.then(unary)
			.repeated(),
			|l, (op, r)| Expr::Bin(Box::new(l), op, Box::new(r)),
		);

		product.clone().foldl(
			choice((
				just(Token::Plus).to(BinOp::Add),
				just(Token::Minus).to(BinOp::Sub),
			))
			.then(product)
			.repeated(),
			|l, (op, r)| Expr::Bin(Box::new(l), op, Box::new(r)),
		)
	});

	// statements split by `;` or newline (any amount of them)
	let sep = just(Token::NewLine)
		.repeated()
		.at_least(1);

	expr.separated_by(sep)
		.allow_leading()
		.allow_trailing()
		.collect::<Vec<_>>()
		.then_ignore(end())
}

pub fn parse_calc(calc_input: SharedString, var_input: SharedString) -> Result<SharedString, SharedString> {
	let tokens = lex(&calc_input);
	let stmts = match parser().parse(tokens.as_slice()).into_result() {
		Ok(s) => s,
		Err(errs) => {
			let err_len = errs.iter().len();
			return Err(SharedString::from(errs.into_iter().enumerate()
				.map(|(idx, err)| format!("{:?}{}", err, if idx < err_len - 1 { "\n" } else { "" }))
				.collect::<String>())
			)
		}
		// Err(errs) => return Err(errs.into_iter().map(|err| SharedString::from(format!("{err}\n"))).collect()),
	};

	let mut env = HashMap::<i64, Value>::new();
	let vars = parse_var(&var_input).unwrap_or(HashMap::<String, Value>::new());

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

	Ok(SharedString::from(env.iter().enumerate()
		.map(|(idx, _)| format!("{}\n", env.get(&((idx + 1) as i64)).unwrap_or(&Value::Invalid)))
		.collect::<String>()
	))
}
