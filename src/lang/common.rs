use logos::Logos;
use chumsky::{ prelude::* };
use std::{ collections::HashMap, f64::consts as MathConst, fmt };
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

	#[token(",")]
	Comma,

	#[token(".")]
	Dot,

	#[regex(r"[0-9]+(_[0-9]+)*\.[0-9]+", |lex| lex.slice().replace("_", "").parse::<f64>().map_err(|_| LexError::NumberOverflow))]
	Float(f64),

	#[regex(r"[0-9]+(_[0-9]+)*", |lex| lex.slice().replace("_", "").parse::<i64>().map_err(|_| LexError::NumberOverflow))]
	Number(i64),

	// functions
	#[token("abs", |_| MathFunction::Abs)]
	#[token("acos", |_| MathFunction::Acos)]
	#[token("acosh", |_| MathFunction::Acosh)]
	#[token("asin", |_| MathFunction::Asin)]
	#[token("asinh", |_| MathFunction::Asinh)]
	#[token("atan", |_| MathFunction::Atan)]
	#[token("atan2", |_| MathFunction::Atan2)]
	#[token("atanh", |_| MathFunction::Atanh)]
	#[token("cbrt", |_| MathFunction::Cbrt)]
	#[token("ceil", |_| MathFunction::Ceil)]
	#[token("cos", |_| MathFunction::Cos)]
	#[token("cosh", |_| MathFunction::Cosh)]
	#[token("exp", |_| MathFunction::Exp)]
	#[token("exp2", |_| MathFunction::Exp2)]
	#[token("floor", |_| MathFunction::Floor)]
	#[token("fract", |_| MathFunction::Fract)]
	#[token("ln", |_| MathFunction::Ln)]
	#[token("log", |_| MathFunction::Log)]
	#[token("log10", |_| MathFunction::Log10)]
	#[token("log2", |_| MathFunction::Log2)]
	#[token("max", |_| MathFunction::Max)]
	#[token("mid", |_| MathFunction::Mid)]
	#[token("min", |_| MathFunction::Min)]
	#[token("pow", |_| MathFunction::Pow)]
	#[token("round", |_| MathFunction::Round)]
	#[token("sign", |_| MathFunction::Sign)]
	#[token("sin", |_| MathFunction::Sin)]
	#[token("sinh", |_| MathFunction::Sinh)]
	#[token("sqrt", |_| MathFunction::Sqrt)]
	#[token("tan", |_| MathFunction::Tan)]
	#[token("tanh", |_| MathFunction::Tanh)]
	#[token("trunc", |_| MathFunction::Trunc)]
	#[token("deg", |_| MathFunction::Deg)]
	#[token("rad", |_| MathFunction::Rad)]
	Func(MathFunction),

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

#[derive(Debug, Clone, PartialEq)]
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

impl MathFunction {
	pub fn arity(&self) -> usize {
		use MathFunction::*;

		match self {
			Atan2 | Log | Max | Min | Pow => 2,
			Mid => 3,
			_ => 1,
		}
	}
}

pub fn get_math_const() -> HashMap<String, Value> {
	HashMap::from([
		("e".into(), Value::Float(MathConst::E)),
		("ln2".into(), Value::Float(MathConst::LN_2)),
		("ln10".into(), Value::Float(MathConst::LN_10)),
		("log2_10".into(), Value::Float(MathConst::LOG2_10)),
		("log2_e".into(), Value::Float(MathConst::LOG2_E)),
		("log10_2".into(), Value::Float(MathConst::LOG10_2)),
		("log10_e".into(), Value::Float(MathConst::LOG10_E)),
		("pi".into(), Value::Float(MathConst::PI)),
		("phi".into(), Value::Float(MathConst::GOLDEN_RATIO)),
		("sqrt_2".into(), Value::Float(MathConst::SQRT_2)),
		("tau".into(), Value::Float(MathConst::TAU)),
	])
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
		let func = select! { Token::Func(f) => f }.labelled("function");

		let args = expr
			.clone()
			.separated_by(just(Token::Comma))
			.allow_trailing()
			.collect::<Vec<_>>()
			.delimited_by(just(Token::LParen), just(Token::RParen));

		let atom = choice((
			select! {
				Token::Number(n) => Expr::Int(n),
				Token::Float(f) => Expr::Float(f),
				Token::Identifier(s) => Expr::Var(s),
				Token::Error => Expr::Invalid,
			},
			func.clone()
				.then(args.clone())
				.try_map(|(f, a), span| {
					if a.len() == f.arity() {
						Ok(Expr::Function(f, a))
					} else {
						Err(Rich::custom(span, format!("{f:?} takes {} args", f.arity())))
					}
				}),
			expr.delimited_by(just(Token::LParen), just(Token::RParen)),
		));

		// `.abs` (no parens) or `.pow(2)`, receiver counts as arg 0
		let method = just(Token::Dot)
			.ignore_then(func)
			.then(args.or_not().map(Option::unwrap_or_default))
			.try_map(|(f, a), span| {
				if a.len() + 1 == f.arity() {
					Ok((f, a))
				} else {
					Err(Rich::custom(span, format!("{f:?} method takes {} args", f.arity() - 1)))
				}
			});

		let postfix = atom.foldl(
			method.repeated(),
			|lhs, (f, mut a)| {
				a.insert(0, lhs);
				Expr::Function(f, a)
			},
		);

		// unary + pow live in one recursive block so `2^-2` works
		let unary = recursive(|unary| {
			let pow = postfix
				.clone()
				.then(
					choice((just(Token::Caret), just(Token::DoubleStar)))
						.ignore_then(unary.clone())
						.or_not(),
				)
				.map(|(l, r)| match r {
					Some(r) => Expr::Bin(Box::new(l), BinOp::Exp, Box::new(r)),
					None => l,
				});

			choice((
				just(Token::Minus)
					.ignore_then(unary.clone())
					.map(|r| Expr::Neg(Box::new(r))),
				just(Token::Hashtag)
					.ignore_then(unary)
					.map(|r| Expr::Sqrt(Box::new(r))),
				pow,
			))
		});

		let product = unary.clone().foldl(
			choice((
				just(Token::Star).to(BinOp::Mul),
				just(Token::Slash).to(BinOp::Div),
				just(Token::Percent).to(BinOp::Mod),
				just(Token::Backslash).to(BinOp::IntDiv),
				just(Token::DoubleSlash).to(BinOp::IntDiv),
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
								Value::Int(n) => Some(Value::Float((n as f64).log10())),
								Value::Float(f) => Some(Value::Float(f.log10())),
								_ => None
							},
							MathFunction::Log2 => match eval(first, env)? {
								Value::Int(n) => Some(Value::Float((n as f64).log2())),
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
							Value::Int(n) => (n as i32, n as f64),
							Value::Float(f) => (f as i32, f),
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
								Value::Int(n) => Some(Value::Float((n as f64).max(sec_float))),
								Value::Float(f) => Some(Value::Float(f.max(sec_float))),
								_ => None
							},
							MathFunction::Mid => match eval(first, env)? {
								Value::Int(n) => Some(Value::Float((n as f64).midpoint(sec_float))),
								Value::Float(f) => Some(Value::Float(f.midpoint(sec_float))),
								_ => None
							},
							MathFunction::Min => match eval(first, env)? {
								Value::Int(n) => Some(Value::Float((n as f64).min(sec_float))),
								Value::Float(f) => Some(Value::Float(f.min(sec_float))),
								_ => None
							},
							MathFunction::Pow => match eval(first, env)? {
								Value::Int(n) => Some(Value::Float((n as f64).powi(sec_int))),
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
