use std::fmt;

use crate::lexer::Token;

#[derive(Debug, Eq, PartialEq)]
pub enum UnaryOp {
	Plus,
	Minus
}

#[derive(Debug, Eq, PartialEq)]
pub enum BinaryOp {
	Plus,
	Minus,
	Multiply,
	Divide
}

impl fmt::Display for BinaryOp {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			BinaryOp::Plus => write!(f, "+"),
			BinaryOp::Minus => write!(f, "-"),
			BinaryOp::Multiply => write!(f, "*"),
			BinaryOp::Divide => write!(f, "/")
		}
	}
}

#[derive(Debug, Eq, PartialEq)]
pub enum AST {
	UnaryOp { token: Token, op: UnaryOp, expr: Box<AST> },
	BinaryOp { token: Token, lhs: Box<AST>, op: BinaryOp, rhs: Box<AST> },
	Number { token: Token, value: u32 }
}

impl AST {
	pub fn unary_op(token: &Token, expr: AST) -> Self {
		let op = match token {
			Token::Plus => UnaryOp::Plus,
			Token::Minus => UnaryOp::Minus,
			_ => panic!("invalid unary op {:?}", token)
		};
		AST::UnaryOp {
			token: token.clone(),
			op,
			expr: Box::new(expr)
		}
	}

	pub fn binary_op(lhs: AST, token: &Token, rhs: AST) -> Self {
		let op = match token {
			Token::Plus => BinaryOp::Plus,
			Token::Minus => BinaryOp::Minus,
			Token::Multiply => BinaryOp::Multiply,
			Token::Divide => BinaryOp::Divide,
			_ => panic!("invalid binary op {:?}", token)
		};
		AST::BinaryOp {
			token: token.clone(),
			lhs: Box::new(lhs),
			op,
			rhs: Box::new(rhs)
		}
	}

	pub fn number(token: &Token, value: u32) -> Self {
		AST::Number {
			token: token.clone(),
			value
		}
	}
}
