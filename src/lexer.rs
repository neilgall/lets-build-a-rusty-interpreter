use std::collections::HashMap;
use std::fmt;
use std::io::*;
use std::iter::Peekable;
use std::str::*;
use std::format;


#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Token {
	IntegerLiteral { value: u32 },
	RealLiteral { value: String }, // lexer keeps reals as string for equality
	Identifier { value: String },
	Program,
	Var,
	Begin,
	End,
	Dot,
	Colon,
	Comma,
	Integer,
	Real,
	Plus,
	Minus,
	Multiply,
	IntegerDivide,
	RealDivide,
	OpenParen,
	CloseParen,
	Assign,
	EndStatement,
	Eof
}

lazy_static! {
	static ref KEYWORD_TOKENS: HashMap<&'static str, Token> = {
		let mut keywords = HashMap::new();
		keywords.insert("PROGRAM", Token::Program);
		keywords.insert("VAR", Token::Var);
		keywords.insert("BEGIN", Token::Begin);
		keywords.insert("END", Token::End);
		keywords.insert("INTEGER", Token::Integer);
		keywords.insert("REAL", Token::Real);
		keywords.insert("DIV", Token::IntegerDivide);
		keywords
	};

	static ref KEYWORD_STRINGS: HashMap<Token, &'static str> = {
		let mut keywords = HashMap::new();
		for (&key, val) in KEYWORD_TOKENS.iter() {
			keywords.insert(val.clone(), key);
		}
		keywords
	};
}


impl fmt::Display for Token {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match KEYWORD_STRINGS.get(self) { 
			Some(s) => write!(f, "{}", s),
			None => match self {
				Token::IntegerLiteral { value } => write!(f, "{}", value.to_string()),
				Token::RealLiteral { value } => write!(f, "{}", value),
				Token::Identifier { value } => write!(f, "{}", value),
				Token::Dot => write!(f, "."),
				Token::Plus => write!(f, "+"),
				Token::Minus => write!(f, "-"),
				Token::Multiply => write!(f, "*"),
				Token::RealDivide => write!(f, "/"),
				Token::OpenParen => write!(f, "("),
				Token::CloseParen => write!(f, ")"),
				Token::Assign => write!(f, ":="),
				Token::EndStatement => write!(f, ";"),
				Token::Eof => Ok(()),
				_ => panic!("no display for {:?}", self)
			}
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

	fn skip_comment(&mut self) {
		while self.current.map_or(false, |c| c != '}') {
			self.advance();
		}
	}

	fn number(&mut self) -> Token {
		let mut value = String::new();
		while self.current.is_some() && self.current.unwrap().is_digit(10) {
			value.push(self.advance());
		}
		if self.current.map_or(false, |c| c == '.') {
			value.push(self.advance());
			while self.current.is_some() && self.current.unwrap().is_digit(10) {
				value.push(self.advance());
			}
			Token::RealLiteral { value }
		} else {
			let i = u32::from_str_radix(&value, 10).unwrap();
			Token::IntegerLiteral { value: i }
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
					'{' => {
						self.advance();
						self.skip_comment();
						self.next_token()
					}
					'.' => {
						self.advance();
						Ok(Token::Dot)
					}
					',' => {
						self.advance();
						Ok(Token::Comma)
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
						Ok(Token::RealDivide)
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
					':' => {
						if self.peek('=') {
							self.advance();
							self.advance();
							Ok(Token::Assign)
						} else {
							self.advance();
							Ok(Token::Colon)
						}
					}
					_ if c.is_digit(10) => {
						Ok(self.number())
					}
					_ if c == '_' || c.is_alphabetic() => {
						let id = self.identifier();
						Ok(match KEYWORD_TOKENS.get(&id.to_uppercase().as_ref()) {
							Some(token) => token.clone(),
							None => Token::Identifier { value: id }
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
	fn keywords() {
		KEYWORD_TOKENS.iter().for_each(|(string, token)| {
			assert_eq!(read(string), token.clone());
			assert_eq!(read(&string.to_lowercase()), token.clone());
		});
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
		assert_eq!(read("123"), Token::IntegerLiteral { value: 123 });
	}

	#[test]
	fn real() {
		assert_eq!(read("123.456"), Token::RealLiteral { value: String::from("123.456") });
	}

	#[test]
	fn assign() {
		assert_eq!(read(":="), Token::Assign);
	}

	#[test]
	fn colon() {
		assert_eq!(read(": foo"), Token::Colon);
	}

}
