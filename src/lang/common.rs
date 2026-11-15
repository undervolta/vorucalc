use logos::Logos;
use chumsky::prelude::*;
use std::{ fmt, collections::HashMap };
use num_format::{ Locale, ToFormattedString };

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

	#[token("^")]
	Caret,

	#[token("**")]
	DoubleStar,

	#[token("*")]
	Star,

	#[token("//")]
	DoubleSlash,

	#[token("/")]
	Slash,

	#[token("\\")]
	Backslash,

	#[token("%")]
	Percent,

	#[token("#")]
	Hashtag,

	#[token(";")]
	Semicolon,

	#[token("\n")]
	NewLine,

	#[token("(")]
	LParen,

	#[token(")")]
	RParen,

	#[regex(r"[0-9]+(_[0-9]+)*\.[0-9]+", |lex| lex.slice().replace("_", "").parse::<f64>().map_err(|_| LexError::NumberOverflow))]
	Float(f64),

	#[regex(r"[0-9]+(_[0-9]+)*", |lex| lex.slice().replace("_", "").parse::<i64>().map_err(|_| LexError::NumberOverflow))]
	Number(i64),

	#[token(",")]
	Comma,

	#[token(".")]
	Dot,

	// functions
	#[token("abs")]
	FnAbs,
	#[token("acos")]
	FnAcos,
	#[token("acosh")]
	FnAcosh,
	#[token("asin")]
	FnAsin,
	#[token("asinh")]
	FnAsinh,
	#[token("atan")]
	FnAtan,
	#[token("atan2")]
	FnAtan2,
	#[token("atanh")]
	FnAtanh,
	#[token("cbrt")]
	FnCbrt,
	#[token("ceil")]
	FnCeil,
	#[token("cos")]
	FnCos,
	#[token("cosh")]
	FnCosh,
	#[token("exp")]
	FnExp,
	#[token("exp2")]
	FnExp2,
	#[token("floor")]
	FnFloor,
	#[token("fract")]
	FnFract,
	#[token("ln")]
	FnLn,
	#[token("log")]
	FnLog,
	#[token("log10")]
	FnLog10,
	#[token("log2")]
	FnLog2,
	#[token("max")]
	FnMax,
	#[token("mid")]
	FnMid,
	#[token("min")]
	FnMin,
	#[token("pow")]
	FnPow,
	#[token("round")]
	FnRound,
	#[token("sign")]
	FnSign,
	#[token("sin")]
	FnSin,
	#[token("sinh")]
	FnSinh,
	#[token("sqrt")]
	FnSqrt,
	#[token("tan")]
	FnTan,
	#[token("tanh")]
	FnTanh,
	#[token("trunc")]
	FnTrunc,
	#[token("deg")]
	FnDeg,
	#[token("rad")]
	FnRad,

	// identifiers
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
	IntDiv,
	Mod,
	Exp,
}

#[derive(Debug, Clone)]
pub enum Expr {
	Int(i64),
	Float(f64),
	Var(String),

	Neg(Box<Expr>),
	Sqrt(Box<Expr>),
	Function(MathFunction, Vec<Expr>),

	Bin(Box<Expr>, BinOp, Box<Expr>),
	Invalid,
}

pub type VarStmt = (String, Expr);


pub fn format_float(value: f64, decimal_prec: usize) -> String {
	let formatted = format!("{:.*}", decimal_prec, value.abs());

    let (integer, fraction) = formatted
        .split_once('.')
        .unwrap_or((&formatted, ""));

    let grouped = integer
        .parse::<u64>()
        .unwrap_or(0)
        .to_formatted_string(&Locale::en);

	let fraction = fraction.trim_end_matches('0');
    let sign = if value.is_sign_negative() { "-" } else { "" };

    if fraction.is_empty() {
        format!("{sign}{grouped}")
    } else {
        format!("{sign}{grouped}.{fraction}")
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Value {
	Int(i64),
	Float(f64),
	Invalid
}

impl Value {
	pub fn as_f64(self) -> f64 {
		match self {
			Value::Int(n) => n as f64,
			Value::Float(f) => f,
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

#[derive(Debug, Clone)]
pub enum MathFunction {
	Abs,
	Acos,
	Acosh,
	Asin,
	Asinh,
	Atan,
	Atan2,
	Atanh,
	Cbrt,
	Ceil,
	Cos,
	Cosh,
	Exp,
	Exp2,
	Floor,
	Fract,
	Ln,
	Log,
	Log10,
	Log2,
	Max,
	Mid,
	Min,
	Pow,
	Round,
	Sign,
	Sin,
	Sinh,
	Sqrt,
	Tan,
	Tanh,
	Trunc,
	Deg,
	Rad,
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
					Some((a as f64) / (b as f64)).map(Value::Float)
				}
			},
			BinOp::Mod => a.checked_rem_euclid(b).map(Value::Int),
			BinOp::IntDiv => {
				if b == 0 {
					None
				} else {
					Some((a as f64).div_euclid(b as f64)).map(Value::Float)
				}
			},
			BinOp::Exp => a.checked_pow(b as u32).map(Value::Int),
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
				BinOp::Mod => a.rem_euclid(b),
				BinOp::IntDiv => a.div_euclid(b),
				BinOp::Exp => a.powf(b),
			}))
		}
	}
}


pub fn parse_token<'a>() -> impl Parser<'a, &'a [Token], Expr, extra::Err<Rich<'a, Token>>,> + Clone {
	recursive(|expr| {
		let atom = select! {
			Token::Number(n) => Expr::Int(n),
			Token::Float(f) => Expr::Float(f),
			Token::Identifier(s) => Expr::Var(s),
			Token::Error => Expr::Invalid,
		}
		.or(expr.clone().delimited_by(just(Token::LParen), just(Token::RParen)));
		
		// single parametered functions
		let math_fn = choice((
				choice((
					just(Token::FnAbs).to(MathFunction::Abs),
					just(Token::FnCeil).to(MathFunction::Ceil),
					just(Token::FnFloor).to(MathFunction::Floor),
					just(Token::FnFract).to(MathFunction::Fract),
					just(Token::FnMax).to(MathFunction::Max),
					just(Token::FnMid).to(MathFunction::Mid),
					just(Token::FnMin).to(MathFunction::Min),
					just(Token::FnPow).to(MathFunction::Pow),
					just(Token::FnRound).to(MathFunction::Round),
					just(Token::FnSign).to(MathFunction::Sign),
					just(Token::FnTrunc).to(MathFunction::Trunc),
				)),
				choice((
					just(Token::FnAcos).to(MathFunction::Acos),
					just(Token::FnAcosh).to(MathFunction::Acosh),
					just(Token::FnAsin).to(MathFunction::Asin),
					just(Token::FnAsinh).to(MathFunction::Asinh),
					just(Token::FnAtan).to(MathFunction::Atan),
					just(Token::FnAtan2).to(MathFunction::Atan2),
					just(Token::FnAtanh).to(MathFunction::Atanh),
					just(Token::FnCbrt).to(MathFunction::Cbrt),
					just(Token::FnCos).to(MathFunction::Cos),
					just(Token::FnCosh).to(MathFunction::Cosh),
					just(Token::FnDeg).to(MathFunction::Deg),
					just(Token::FnExp).to(MathFunction::Exp),
					just(Token::FnExp2).to(MathFunction::Exp2),				
					just(Token::FnLn).to(MathFunction::Ln),
					just(Token::FnLog).to(MathFunction::Log),
					just(Token::FnLog10).to(MathFunction::Log10),
					just(Token::FnLog2).to(MathFunction::Log2),
					just(Token::FnRad).to(MathFunction::Rad),
					just(Token::FnSin).to(MathFunction::Sin),
					just(Token::FnSinh).to(MathFunction::Sinh),
					just(Token::FnSqrt).to(MathFunction::Sqrt),
					just(Token::FnTan).to(MathFunction::Tan),
					just(Token::FnTanh).to(MathFunction::Tanh)
				))
			))
			.then(expr
				.separated_by(just(Token::Comma))
				.allow_trailing()
				.collect::<Vec<_>>()
				.delimited_by(just(Token::LParen), just(Token::RParen)))
			.map(|(math_fn, expr)| Expr::Function(math_fn, expr));

		let atom = atom.or(math_fn);

		let unary = just(Token::Minus)
			.repeated()
			.foldr(atom, |_, rhs| Expr::Neg(Box::new(rhs)));

		let sqrt = just(Token::Hashtag)
			.repeated()
			.foldr(unary, |_, rhs| Expr::Sqrt(Box::new(rhs)));

		let product = sqrt.clone().foldl(
			choice((
				just(Token::Caret).to(BinOp::Exp),
				just(Token::DoubleStar).to(BinOp::Exp),
			))
			.then(sqrt)
			.repeated(),
			|l, (op, r)| Expr::Bin(Box::new(l), op, Box::new(r)),
		);

		let product = product.clone().foldl(
			choice((
				just(Token::Star).to(BinOp::Mul),
				just(Token::Slash).to(BinOp::Div),
				just(Token::Percent).to(BinOp::Mod),
				just(Token::Backslash).to(BinOp::IntDiv),
				just(Token::DoubleSlash).to(BinOp::IntDiv),
			))
			.then(product)
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
	})
}

// None = invalid
pub fn eval(e: &Expr, env: &HashMap<String, Value>) -> Option<Value> {
	match e {
		Expr::Invalid => None,
		Expr::Int(n) => Some(Value::Int(*n)),
		Expr::Float(f) => Some(Value::Float(*f)),
		Expr::Var(name) => env.get(name).copied(),
		Expr::Neg(inner) => match eval(inner, env)? {
			Value::Int(n) => n.checked_neg().map(Value::Int),
			Value::Float(f) => Some(Value::Float(-f)),
			Value::Invalid => None
		},
		Expr::Function(math_fn, args) => {
			match &args.len() {
				1 => {
					if let [first] = &args[..1] {
						match math_fn {
							MathFunction::Abs => match eval(first, env)? {
								Value::Int(n) => n.checked_abs().map(Value::Int),
								Value::Float(f) => Some(Value::Float(f.abs())),
								_ => None
							},
							MathFunction::Acos => match eval(first, env)? {
								Value::Int(n) => Some(Value::Float((n as f64).acos())),
								Value::Float(f) => Some(Value::Float(f.acos())),
								_ => None
							},
							MathFunction::Acosh => match eval(first, env)? {
								Value::Int(n) => Some(Value::Float((n as f64).acosh())),
								Value::Float(f) => Some(Value::Float(f.acosh())),
								_ => None
							},
							MathFunction::Asin => match eval(first, env)? {
								Value::Int(n) => Some(Value::Float((n as f64).asin())),
								Value::Float(f) => Some(Value::Float(f.asin())),
								_ => None
							},
							MathFunction::Asinh => match eval(first, env)? {
								Value::Int(n) => Some(Value::Float((n as f64).asinh())),
								Value::Float(f) => Some(Value::Float(f.asinh())),
								_ => None
							},
							MathFunction::Atan => match eval(first, env)? {
								Value::Int(n) => Some(Value::Float((n as f64).atan())),
								Value::Float(f) => Some(Value::Float(f.atan())),
								_ => None
							},
							MathFunction::Atanh => match eval(first, env)? {
								Value::Int(n) => Some(Value::Float((n as f64).atanh())),
								Value::Float(f) => Some(Value::Float(f.atanh())),
								_ => None
							},
							MathFunction::Cbrt => match eval(first, env)? {
								Value::Int(n) => Some(Value::Float((n as f64).cbrt())),
								Value::Float(f) => Some(Value::Float(f.cbrt())),
								_ => None
							},
							MathFunction::Ceil => match eval(first, env)? {
								Value::Int(n) => Some(Value::Int(n)),
								Value::Float(f) => Some(Value::Float(f.ceil())),
								_ => None
							},
							MathFunction::Cos => match eval(first, env)? {
								Value::Int(n) => Some(Value::Float((n as f64).cos())),
								Value::Float(f) => Some(Value::Float(f.cos())),
								_ => None
							},
							MathFunction::Cosh => match eval(first, env)? {
								Value::Int(n) => Some(Value::Float((n as f64).cosh())),
								Value::Float(f) => Some(Value::Float(f.cosh())),
								_ => None
							},
							MathFunction::Deg => match eval(first, env)? {
								Value::Int(n) => Some(Value::Float((n as f64).to_degrees())),
								Value::Float(f) => Some(Value::Float(f.to_degrees())),
								_ => None
							},
							MathFunction::Exp => match eval(first, env)? {
								Value::Int(n) => Some(Value::Float((n as f64).exp())),
								Value::Float(f) => Some(Value::Float(f.exp())),
								_ => None
							},
							MathFunction::Exp2 => match eval(first, env)? {
								Value::Int(n) => Some(Value::Float((n as f64).exp2())),
								Value::Float(f) => Some(Value::Float(f.exp2())),
								_ => None
							},
							MathFunction::Floor => match eval(first, env)? {
								Value::Int(n) => Some(Value::Int(n)),
								Value::Float(f) => Some(Value::Float(f.floor())),
								_ => None
							},
							MathFunction::Fract => match eval(first, env)? {
								Value::Int(n) => Some(Value::Float((n as f64).fract())),
								Value::Float(f) => Some(Value::Float(f.fract())),
								_ => None
							},
							MathFunction::Ln => match eval(first, env)? {
								Value::Int(n) => Some(Value::Float((n as f64).ln())),
								Value::Float(f) => Some(Value::Float(f.ln())),
								_ => None
							},
							MathFunction::Log10 => match eval(first, env)? {
								Value::Int(n) => Some(Value::Int(n.ilog10().into())),
								Value::Float(f) => Some(Value::Float(f.log10())),
								_ => None
							},
							MathFunction::Log2 => match eval(first, env)? {
								Value::Int(n) => Some(Value::Int(n.ilog10().into())),
								Value::Float(f) => Some(Value::Float(f.log2())),
								_ => None
							},
							MathFunction::Rad => match eval(first, env)? {
								Value::Int(n) => Some(Value::Float((n as f64).to_radians())),
								Value::Float(f) => Some(Value::Float(f.to_radians())),
								_ => None
							},
							MathFunction::Round => match eval(first, env)? {
								Value::Int(n) => Some(Value::Int(n)),
								Value::Float(f) => Some(Value::Float(f.round())),
								_ => None
							},
							MathFunction::Sign => match eval(first, env)? {
								Value::Int(n) => Some(Value::Int(n.signum())),
								Value::Float(f) => Some(Value::Float(f.signum())),
								_ => None
							},
							MathFunction::Sin => match eval(first, env)? {
								Value::Int(n) => Some(Value::Float((n as f64).sin())),
								Value::Float(f) => Some(Value::Float(f.sin())),
								_ => None
							},
							MathFunction::Sinh => match eval(first, env)? {
								Value::Int(n) => Some(Value::Float((n as f64).sinh())),
								Value::Float(f) => Some(Value::Float(f.sinh())),
								_ => None
							},
							MathFunction::Sqrt => match eval(first, env)? {
								Value::Int(n) => n.checked_isqrt().map(Value::Int),
								Value::Float(f) => Some(Value::Float(f.sqrt())),
								_ => None
							},
							MathFunction::Tan => match eval(first, env)? {
								Value::Int(n) => Some(Value::Float((n as f64).tan())),
								Value::Float(f) => Some(Value::Float(f.tan())),
								_ => None
							},
							MathFunction::Tanh => match eval(first, env)? {
								Value::Int(n) => Some(Value::Float((n as f64).tanh())),
								Value::Float(f) => Some(Value::Float(f.tanh())),
								_ => None
							},
							MathFunction::Trunc => match eval(first, env)? {
								Value::Int(n) => Some(Value::Int(n)),
								Value::Float(f) => Some(Value::Float(f.trunc())),
								_ => None
							},
							_ => None
						}
					} else { None }
				},
				2 => {
					if let [first, second] = &args[..2] {
						let (sec_int, sec_float) = match eval(second, env)? {
							Value::Int(n) => (n, 0.0),
							Value::Float(f) => (0, f),
							Value::Invalid => (0, 0.0)
						};

						match math_fn {
							MathFunction::Atan2 => match eval(first, env)? {
								Value::Int(n) => Some(Value::Float((n as f64).atan2(sec_float))),
								Value::Float(f) => Some(Value::Float(f.atan2(sec_float))),
								_ => None
							},
							MathFunction::Log => match eval(first, env)? {
								Value::Int(n) => Some(Value::Float((n as f64).log(sec_float))),
								Value::Float(f) => Some(Value::Float(f.log(sec_float))),
								_ => None
							},
							MathFunction::Max => match eval(first, env)? {
								Value::Int(n) => Some(Value::Int(n.max(sec_int))),
								Value::Float(f) => Some(Value::Float(f.max(sec_float))),
								_ => None
							},
							MathFunction::Mid => match eval(first, env)? {
								Value::Int(n) => Some(Value::Int(n.midpoint(sec_int))),
								Value::Float(f) => Some(Value::Float(f.midpoint(sec_float))),
								_ => None
							},
							MathFunction::Min => match eval(first, env)? {
								Value::Int(n) => Some(Value::Int(n.min(sec_int))),
								Value::Float(f) => Some(Value::Float(f.min(sec_float))),
								_ => None
							},
							MathFunction::Pow => match eval(first, env)? {
								Value::Int(n) => Some(Value::Int(n.pow(sec_int as u32))),
								Value::Float(f) => Some(Value::Float(f.powf(sec_float))),
								_ => None
							},
							_ => None
						}
					} else { None }
				},
				_ => None
			}
		}, 
		Expr::Sqrt(inner) => eval(&Expr::Function(MathFunction::Sqrt, vec![*inner.to_owned()]), env),
		Expr::Bin(l, op, r) => apply(eval(l, env)?, *op, eval(r, env)?),
	}
}
