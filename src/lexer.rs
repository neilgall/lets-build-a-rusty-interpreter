use std::fmt;
use std::io::*;
use std::iter::Peekable;
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
	Assign,
	EndStatement,
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
			Token::Assign => write!(f, ":="),
			Token::EndStatement => write!(f, ";"),
			Token::Eof => Ok(())
		}
	}
}

pub struct Lexer<'a> {
	text: Peekable<Chars<'a>>,
	current: Option<char>
}

pub fn invalid<A>(msg: &str) -> Result<A> {
	Err(Error::new(ErrorKind::InvalidData, msg))
}

impl<'a> Lexer<'a> {
	pub fn new(text: &'a str) -> Self {
		let mut text = text.chars().peekable();
		let current = text.next();
		Lexer { text, current }
	}

	fn advance(& mut self) -> char {
		let c = self.current.unwrap();
		self.current = self.text.next();
		c
	}

	fn peek(&mut self, c: char) -> bool {
		self.text.peek() == Some(&c)
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
		if self.current.map_or(false, |c| c == '_' || c.is_alphabetic()) {
			result.push(self.advance());
		}
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
					'(' => {
						self.advance();
						Ok(Token::OpenParen)
					}
					')' => {
						self.advance();
						Ok(Token::CloseParen)
					}
					';' => {
						self.advance();
						Ok(Token::EndStatement)
					}
					':' if self.peek('=') => {
						self.advance();
						self.advance();
						Ok(Token::Assign)
					}
					_ if c.is_digit(10) => {
						Ok(Token::Integer { value: self.integer() })
					}
					_ if c == '_' || c.is_alphabetic() => {
						let id = self.identifier();
						Ok(match id.to_uppercase().as_ref() {
							"BEGIN" => Token::Begin,
							"END" => Token::End,
							"DIV" => Token::Divide,
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

#[cfg(test)]
mod tests {
	use super::*;

	fn read(s: &str) -> Token {
		let mut lex = Lexer::new(s);
		lex.next_token().unwrap()
	}

	#[test]
	fn dot() {
		assert_eq!(read("."), Token::Dot);
	}

	#[test]
	fn begin() {
		assert_eq!(read("BEGIN"), Token::Begin);
		assert_eq!(read("begin"), Token::Begin);
		assert_eq!(read("BeGiN"), Token::Begin);
	}

	#[test]
	fn end() {
		assert_eq!(read("END"), Token::End);
		assert_eq!(read("end"), Token::End);
		assert_eq!(read("EnD"), Token::End);
	}

	#[test]
	fn identifier() {
		assert_eq!(read("foo"), Token::Identifier { value: String::from("foo") });
	}

	#[test]
	fn alphanumeric_identifier() {
		assert_eq!(read("foo42bar"), Token::Identifier { value: String::from("foo42bar") });
	}

	#[test]
	fn integer() {
		assert_eq!(read("123"), Token::Integer { value: 123 });
	}

	#[test]
	fn assign() {
		assert_eq!(read(":="), Token::Assign);
	}

}
