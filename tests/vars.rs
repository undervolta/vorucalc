use logos::Logos;
use chumsky::{
	prelude::*
};
use std::{ fmt, collections::HashMap };

#[derive(Logos, Debug, PartialEq, Clone)]
#[logos(skip r"[ \t\r]+")]
#[logos(error = LexError)]
enum Token {
	#[token("=")]
	Equal,

	#[token("+")]
	Plus,

	#[token("-")]
	Minus,

	#[token("*")]
	Star,

	#[token("/")]
	Slash,

	#[token(";")]
	Semicolon,

	#[token(r"\n")]
	NewLine,

	#[token("(")]
	LParen,

	#[token(")")]
	RParen,

	#[regex(r"[0-9]+\.[0-9]+", |lex| lex.slice().parse::<f64>().map_err(|_| LexError::NumberOverflow))]
	Float(f64),

	#[regex(r"[0-9]+", |lex| lex.slice().parse::<i64>().map_err(|_| LexError::NumberOverflow))]
	Number(i64),

	// Identifiers
	#[regex(r"[a-zA-Z_][a-zA-Z0-9_]*", |lex| lex.slice().to_owned())]
	Identifier(String),

	Error
}

#[derive(Debug, Clone, PartialEq, Default)]
enum LexError {
	NumberOverflow,

	#[default]
	UnknownToken,
}

#[derive(Debug, Clone, Copy)]
enum BinOp {
	Add,
	Sub,
	Mul,
	Div,
}

#[derive(Debug, Clone)]
enum Expr {
	Int(i64),
	Float(f64),
	Var(String),
	Neg(Box<Expr>),
	Bin(Box<Expr>, BinOp, Box<Expr>),
	Invalid,
}

type Stmt = (String, Expr);

fn parser<'a>() -> impl Parser<'a, &'a [Token], Vec<Stmt>, extra::Err<Rich<'a, Token>>> {
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

fn round_to_places(val: f64, places: i32) -> f64 {
	let factor = 10.0_f64.powi(places);
	(val * factor).round() / factor
}

#[derive(Debug, Clone, Copy)]
pub enum Value {
	Int(i64),
	Float(f64),
	Invalid
}

impl Value {
	pub fn as_f64(self) -> f64 {
		match self {
			Value::Int(n) => n as f64,
			Value::Float(f) => round_to_places(f, 3),
			Value::Invalid => 0.0
		}
	}
}

impl fmt::Display for Value {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Value::Int(n) => write!(f, "{n}"),
			Value::Float(x) => write!(f, "{x}"),
			Value::Invalid => write!(f, "?")
		}
	}
}

fn apply(l: Value, op: BinOp, r: Value) -> Option<Value> {
	match (l, r) {
		(Value::Int(a), Value::Int(b)) => match op {
			BinOp::Add => a.checked_add(b),
			BinOp::Sub => a.checked_sub(b),
			BinOp::Mul => a.checked_mul(b),
			BinOp::Div => a.checked_div(b), // None on div by zero
		}
		.map(Value::Int),
		_ => {
			let (a, b) = (l.as_f64(), r.as_f64());
			Some(Value::Float(match op {
				BinOp::Add => a + b,
				BinOp::Sub => a - b,
				BinOp::Mul => a * b,
				BinOp::Div => a / b,
			}))
		}
	}
}

// None = invalid / unknown var / overflow -> prints `?`
fn eval(e: &Expr, env: &HashMap<String, Value>) -> Option<Value> {
	match e {
		Expr::Int(n) => Some(Value::Int(*n)),
		Expr::Float(f) => Some(Value::Float(*f)),
		Expr::Var(name) => env.get(name).copied(),
		Expr::Invalid => None,
		Expr::Neg(inner) => match eval(inner, env)? {
			Value::Int(n) => n.checked_neg().map(Value::Int),
			Value::Float(f) => Some(Value::Float(-f)),
			Value::Invalid => None
		},
		Expr::Bin(l, op, r) => apply(eval(l, env)?, *op, eval(r, env)?),
	}
}

fn lex(src: &str) -> Vec<Token> {
	Token::lexer(src)
		.map(|res| res.unwrap_or(Token::Error))
		.collect()
}

pub fn run(src: &str) -> Result<HashMap<String, Value>, String> {
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

pub const INPUT: &str = r"ten = 10; point = 21.5 - 1 + 7.5;\n par = 2 * (5 + (3 + 1) / 2)\n err = 999999999999999999999999999; from_var = ten + 5 * 3";

// cargo +nightly test --test vars parse -- --no-capture

#[test]
fn parse() {
	let parse_res = run(INPUT);

	if let Ok(map) = parse_res {
		for (key, val) in map {
			println!("Var: {key} = {val}");
		}
	} else {
		println!("Error: {:?}", parse_res.err());
	}
}

// pub mod vars {
//     use crate::{INPUT, run, Value};
// 	use std::collections::HashMap;

// 	pub fn get_vars() -> Result<HashMap<String, Value>, String> {
// 		run(INPUT)
// 	}
// }
