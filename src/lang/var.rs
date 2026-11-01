use chumsky::{
	prelude::*
};
use std::collections::HashMap;
use slint::{ SharedString };

use crate::lang::common::{ 
	Token, BinOp, Expr, Value, VarStmt,
	lex, eval
};


fn parser<'a>() -> impl Parser<'a, &'a [Token], Vec<VarStmt>, extra::Err<Rich<'a, Token>>> {
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
		Err(errs) => return Err(errs.into_iter().map(|err| format!("{err:?}\n")).collect()),
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
