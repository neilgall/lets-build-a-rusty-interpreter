use std::io::Result;
use crate::ast::AST;
use crate::lexer::*;

pub struct Parser<'a> {
	lexer: &'a mut Lexer<'a>,
	current_token: Token
}

impl<'a> Parser<'a> {
	pub fn new(lexer: &'a mut Lexer<'a>) -> Result<Self> {
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
			_ => {
				invalid("expected integer")
			}
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
		self.expr()
	}
}

