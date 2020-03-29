use std::collections::HashMap;

use crate::ast::*;

#[derive(Debug)]
pub struct Scope {
	symbols: HashMap<String, i32>
}

impl Scope {
	pub fn new() -> Self {
		Scope {
			symbols: HashMap::new()
		}
	}

	fn assign(&mut self, name: &str, value: i32) {
		self.symbols.insert(String::from(name), value);
	}

	fn get(&self, name: &str) -> i32 {
		self.symbols[name]
	}
}

pub fn interpret(ast: &AST, scope: &mut Scope) -> i32 {
	match ast {
		AST::Compound { children } => {
			// return the value of the last statement
			children.iter().fold(0, |_, c| interpret(c, scope))
		}

		AST::Assign { token: _, left, right } => {
			let rhs_value = interpret(&right, scope);
			scope.assign(left, rhs_value);
			rhs_value
		}

		AST::UnaryOp { token: _, op, expr } => {
			let expr_value = interpret(&expr, scope);
			match op {
				UnaryOp::Plus => expr_value,
				UnaryOp::Minus => -expr_value
			}
		}
	
		AST::BinaryOp { token: _, lhs, op ,rhs } => {
			let lhs_value = interpret(&lhs, scope);
			let rhs_value = interpret(&rhs, scope);
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

		AST::Variable { token: _, value } => {
			scope.get(value)
		}

		// _ => panic!("not implemented")
	}
}

