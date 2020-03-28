use std::io::Result;
use crate::ast::AST;
use crate::lexer::*;

pub struct Parser<'a> {
	lexer: Lexer<'a>,
	current_token: Token
}

impl<'a> Parser<'a> {
	pub fn new(mut lexer: Lexer<'a>) -> Result<Self> {
		let token = lexer.next_token()?;
		Ok(Parser {
			lexer,
			current_token: token
		})
	}

	fn advance(&mut self) -> Result<Token> {
		let token = self.current_token.clone();
		self.current_token = self.lexer.next_token()?;
		Ok(token)
	}

	fn eat(&mut self, token: &Token) -> Result<()> {
		if &self.current_token == token {
			self.advance()?;
			Ok(())
		} else {
			invalid(&format!("expected {:?}", token))
		}
	}

	fn program(&mut self) -> Result<AST> {
		let node = self.compound_statement()?;
		self.eat(&Token::Dot)?;
		Ok(node)
	}

	fn compound_statement(&mut self) -> Result<AST> {
		self.eat(&Token::Begin)?;
		let statements = self.statement_list()?;
		self.eat(&Token::End)?;
		Ok(AST::compound(statements))
	}

	fn statement_list(&mut self) -> Result<Vec<AST>> {
		let mut nodes = vec![];
		loop {
			match self.current_token {
				Token::EndStatement => { self.advance()?; }
				Token::End => break Ok(nodes),
				_ => { nodes.push(self.statement()?); }
			}
		}
	}

	fn statement(&mut self) -> Result<AST> {
		match self.current_token {
			Token::Begin => {
				self.compound_statement()
			}
			Token::Identifier { value: _ } => {
				self.assignment_statement()
			}
			_ => invalid("expected a statement")
		}
	}

	fn assignment_statement(&mut self) -> Result<AST> {
		let left = match self.variable()? {
			AST::Variable { token: _, value } => Ok(value),
			_ => invalid("must have a variable on LHS of assignment")
		}?;
		let token = self.current_token.clone();
		self.eat(&Token::Assign)?;
		let right = self.expr()?;
		Ok(AST::assign(&token, &left, right))
	}

	fn variable(&mut self) -> Result<AST> {
		let token = self.current_token.clone();
		match token {
			Token::Identifier { value } => {
				self.advance()?;
				Ok(AST::variable(&self.current_token, &value))
			}
			_ => invalid("expected an identifier")
		}
	}

	fn factor(&mut self) -> Result<AST> {
		match self.current_token {
			Token::Plus | Token::Minus => {
				let token = self.advance()?;
				Ok(AST::unary_op(&token, self.expr()?))
			}
			Token::OpenParen => {
				self.advance()?;
				let result = self.expr()?;
				self.eat(&Token::CloseParen)?;
				Ok(result)
			}
			Token::Integer { value } => {
				let token = self.advance()?;
				Ok(AST::number(&token, value))
			}
			_ => self.variable()
		}
	}

	fn term(&mut self) -> Result<AST> {
		let mut node = self.factor()?;
		loop {
			match self.current_token {
				Token::Multiply | Token::Divide => {
					let token = self.advance()?;
					node = AST::binary_op(node, &token, self.factor()?);
				}
				_ => {
					break Ok(node)
				}
			}
		}
	}

	fn expr(&mut self) -> Result<AST> {
		let mut node = self.term()?;
		loop {
			match self.current_token {
				Token::Plus | Token::Minus => {
					let token = self.advance()?;
					node = AST::binary_op(node, &token, self.term()?);
				}
				_ => {
					break Ok(node)
				}
			}
		}
	}

	pub fn parse(&mut self) -> Result<AST> {
		let node = self.program()?;
		self.eat(&Token::Eof)?;
		Ok(node)
	}
}


#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_sample_program() {
		let lexer = Lexer::new("
			BEGIN
				BEGIN
					number := 2;
					a := nunber;
					b := 10 * a + 10 * number / 4;
					c := a - - b;
				END;
				x := 11;
			END.
			");
		let mut parser = Parser::new(lexer).unwrap();
		let result = parser.parse();
		assert!(result.is_ok(), "parser should succeed {:?}", result)
	}
}
