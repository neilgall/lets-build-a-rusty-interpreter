
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
		_ => panic!("not implemented")
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
		_ => panic!("not implemented")
	}
}
