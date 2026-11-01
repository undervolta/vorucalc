use logos::Logos;
use std::{ fmt, collections::HashMap };

#[derive(Logos, Debug, PartialEq, Clone)]
#[logos(skip r"[ \t\r]+")]
#[logos(error = LexError)]
pub enum Token {
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

	#[token("\n")]
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
pub enum LexError {
	NumberOverflow,

	#[default]
	UnknownToken,
}

#[derive(Debug, Clone, Copy)]
pub enum BinOp {
	Add,
	Sub,
	Mul,
	Div,
}

#[derive(Debug, Clone)]
pub enum Expr {
	Int(i64),
	Float(f64),
	Var(String),
	Neg(Box<Expr>),
	Bin(Box<Expr>, BinOp, Box<Expr>),
	Invalid,
}

pub type VarStmt = (String, Expr);

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

pub fn lex(src: &str) -> Vec<Token> {
	Token::lexer(src)
		.map(|res| res.unwrap_or(Token::Error))
		.collect()
}

fn apply(l: Value, op: BinOp, r: Value) -> Option<Value> {
	match (l, r) {
		(Value::Int(a), Value::Int(b)) => match op {
			BinOp::Add => a.checked_add(b).map(Value::Int),
			BinOp::Sub => a.checked_sub(b).map(Value::Int),
			BinOp::Mul => a.checked_mul(b).map(Value::Int),
			BinOp::Div => {
				if b == 0 {
					None
				} else {
					Some(round_to_places(a as f64 / b as f64, 3)).map(Value::Float)
				}
			},
		},

		(Value::Invalid, Value::Int(_)) | 
		(Value::Int(_), Value::Invalid) |
		(Value::Invalid, Value::Float(_)) | 
		(Value::Float(_), Value::Invalid) => None,

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

// None = invalid
pub fn eval(e: &Expr, env: &HashMap<String, Value>) -> Option<Value> {
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
