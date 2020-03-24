use std::io::*;
use std::str::*;
use std::format;

#[derive(Debug, Eq, PartialEq)]
enum Token {
	Integer { value: u32 },
	Plus,
	Minus,
	Multiply,
	Divide,
	Eof
}

struct Lexer<'a> {
	text: Chars<'a>,
	current: Option<char>
}

fn invalid<A>(msg: &str) -> Result<A> {
	Err(Error::new(ErrorKind::InvalidData, msg))
}

impl<'a> Lexer<'a> {
	fn new(text: &'a str) -> Self {
		let mut text = text.chars();
		let current = text.next();
		Lexer { text, current }
	}

	fn advance(& mut self) {
		self.current = self.text.next();
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

	fn next_token(&mut self) -> Result<Token> {
		self.skip_whitespace();
		match self.current {
			None => 
				Ok(Token::Eof),
			
			Some(c) =>
				match c {
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
					_ if c.is_digit(10) => {
						Ok(Token::Integer { value: self.integer() })
					}
					_ => { 
						invalid(&format!("invalid character '{}'", c))
					}
				}
		}
	}
}

fn integer(t: &Token) -> Result<u32> {
	match t {
		Token::Integer { value } => Ok(*value),
		_                        => invalid("expected an integer")
	}
}

fn eat(t: &Token, e: &Token) -> Result<()> {
	if t == e {
		Ok(())
	} else {
		invalid(&format!("expected {:?}", e))
	}
}

fn interpret(text: &str) -> Result<u32> {
	let mut lexer = Lexer::new(&text);
	let left = integer(&lexer.next_token()?)?;
	let op = lexer.next_token()?;
	let right = integer(&lexer.next_token()?)?;

	match op {
		Token::Plus => Ok(left + right),
		Token::Minus => Ok(left - right),
		Token::Multiply => Ok(left * right),
		Token::Divide => Ok(left / right),
		_ => invalid("expected an operator")
	}
}

fn main() -> std::io::Result<()> {
	loop {
		let mut line = String::new();
		println!("Enter an expression");
		stdin().read_line(&mut line)?;
		println!("Result = {}", interpret(&line)?.to_string());
	}
}
