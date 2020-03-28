use std::io::*;
use std::format;

mod ast;
mod lexer;
mod parser;
use ast::*;

fn interpret(ast: &AST) -> i32 {
	match ast {
		AST::UnaryOp { token: _, op, expr } => {
			let expr_value = interpret(&expr);
			match op {
				UnaryOp::Plus => expr_value,
				UnaryOp::Minus => -expr_value
			}
		}
		AST::BinaryOp { token: _, lhs, op ,rhs } => {
			let lhs_value = interpret(&lhs);
			let rhs_value = interpret(&rhs);
			match op {
				BinaryOp::Plus => lhs_value + rhs_value,
				BinaryOp::Minus => lhs_value - rhs_value,
				BinaryOp::Multiply => lhs_value * rhs_value,
				BinaryOp::Divide => lhs_value / rhs_value,
			}
		}
		AST::Number { token: _, value } => {
			*value as i32
		}
	}
}

fn to_postfix(ast: &AST) -> String {
	match ast {
		AST::Number { token: _, value } => {
			value.to_string()
		}
		AST::UnaryOp { token: _, op, expr } => {
			let expr_value = to_postfix(&expr);
			match op {
				UnaryOp::Plus => expr_value,
				UnaryOp::Minus => format!("{} neg", expr_value)
			}
		}
		AST::BinaryOp { token: _, lhs, op, rhs } => {
			let lhs_value = to_postfix(&lhs);
			let rhs_value = to_postfix(&rhs);
			format!("{} {} {}", lhs_value, rhs_value, op)
		}
	}
}

fn to_s_expr(ast: &AST) -> String {	
	match ast {
		AST::Number { token: _, value } => {
			value.to_string()
		}
		AST::UnaryOp { token: _, op, expr } => {
			let expr_value = to_s_expr(&expr);
			match op {
				UnaryOp::Plus => expr_value,
				UnaryOp::Minus => format!("(neg {})", expr_value)
			}
		}
		AST::BinaryOp { token: _, lhs, op, rhs } => {
			let lhs_value = to_s_expr(&lhs);
			let rhs_value = to_s_expr(&rhs);
			format!("({} {} {})", op, lhs_value, rhs_value)
		}
	}
}

fn main() -> std::io::Result<()> {
	loop {
		let mut line = String::new();
		println!("Enter an expression");
		stdin().read_line(&mut line)?;
		let mut lexer = lexer::Lexer::new(&line);
		let mut parser = parser::Parser::new(&mut lexer)?;
		let ast = parser.parse()?;
		println!("postfix: {}", to_postfix(&ast));
		println!("s_expr:  {}", to_s_expr(&ast));
		println!("result:  {}", interpret(&ast));
	}
}
