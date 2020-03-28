use std::fmt;
use std::io::*;
use std::str::*;
use std::format;


#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Token {
	Integer { value: u32 },
	Identifier { value: String },
	Begin,
	End,
	Dot,
	Plus,
	Minus,
	Multiply,
	Divide,
	OpenParen,
	CloseParen,
	Eof
}


impl fmt::Display for Token {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Token::Integer { value } => write!(f, "{}", value.to_string()),
			Token::Identifier { value } => write!(f, "{}", value),
			Token::Begin => write!(f, "BEGIN"),
			Token::End => write!(f, "END"),
			Token::Dot => write!(f, "."),
			Token::Plus => write!(f, "+"),
			Token::Minus => write!(f, "-"),
			Token::Multiply => write!(f, "*"),
			Token::Divide => write!(f, "/"),
			Token::OpenParen => write!(f, "("),
			Token::CloseParen => write!(f, ")"),
			Token::Eof => Ok(())
		}
	}
}

pub struct Lexer<'a> {
	text: Chars<'a>,
	current: Option<char>
}

pub fn invalid<A>(msg: &str) -> Result<A> {
	Err(Error::new(ErrorKind::InvalidData, msg))
}

impl<'a> Lexer<'a> {
	pub fn new(text: &'a str) -> Self {
		let mut text = text.chars();
		let current = text.next();
		Lexer { text, current }
	}

	fn advance(& mut self) -> char {
		let c = self.current.unwrap();
		self.current = self.text.next();
		c
	}

	fn skip_whitespace(&mut self) {		
		while self.current.map_or(false, |c| c.is_whitespace()) {
			self.advance();
		}
	}

	fn integer(&mut self) -> u32 {
		let mut i = 0;
		loop {
			match self.current.and_then(|c| c.to_digit(10)) { 
				None => {
					return i;
				}

				Some(d) => {
					i = (i * 10) + d;
					self.advance();
				}
			}
		}
	}

	fn identifier(&mut self) -> String {
		let mut result: String = String::new();
		while self.current.map_or(false, |c| c.is_alphanumeric()) {
			result.push(self.advance());
		}
		result
	}

	pub fn next_token(&mut self) -> Result<Token> {
		self.skip_whitespace();
		match self.current {
			None => 
				Ok(Token::Eof),
			
			Some(c) =>
				match c {
					'.' => {
						self.advance();
						Ok(Token::Dot)
					}
					'+' => {
						self.advance();
						Ok(Token::Plus)
					}
					'-' => {
						self.advance();
						Ok(Token::Minus)
					}
					'*' => {
						self.advance();
						Ok(Token::Multiply)
					}
					'/' => {
						self.advance();
						Ok(Token::Divide)
					}
					'(' => {
						self.advance();
						Ok(Token::OpenParen)
					}
					')' => {
						self.advance();
						Ok(Token::CloseParen)
					}
					_ if c.is_digit(10) => {
						Ok(Token::Integer { value: self.integer() })
					}
					_ if c.is_alphabetic() => {
						let id = self.identifier();
						Ok(match id.as_ref() {
							"BEGIN" => Token::Begin,
							"END" => Token::End,
							_ => Token::Identifier { value: id }
						})
					}
					_ => { 
						invalid(&format!("invalid character '{}'", c))
					}
				}
		}
	}
}
