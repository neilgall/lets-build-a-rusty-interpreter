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

struct Interpreter<'a> {
	lexer: &'a mut Lexer<'a>,
	current_token: Token
}

impl<'a> Interpreter<'a> {
	fn new(lexer: &'a mut Lexer<'a>) -> Result<Self> {
		let token = lexer.next_token()?;
		Ok(Interpreter {
			lexer,
			current_token: token
		})
	}

	fn advance(&mut self) -> Result<()> {
		self.current_token = self.lexer.next_token()?;
		Ok(())
	}

	fn factor(&mut self) -> Result<u32> {
		match self.current_token {
			Token::Integer { value } => {
				self.advance()?;
				Ok(value)
			}
			_ => {
				invalid("expected integer")
			}
		}
	}

	fn term(&mut self) -> Result<u32> {
		let mut result = self.factor()?;
		loop {
			match self.current_token {
				Token::Multiply => {
					self.advance()?;
					result = result * self.factor()?;
				}
				Token::Divide => {
					self.advance()?;
					result = result / self.factor()?;
				}
				_ => {
					break Ok(result)
				}
			}
		}
	}

	fn expr(&mut self) -> Result<u32> {
		let mut result = self.term()?;
		loop {
			match self.current_token {
				Token::Eof => {
					break Ok(result)
				}
				Token::Plus => {
					self.advance()?;
					result = result + self.term()?;
				}
				Token::Minus => {
					self.advance()?;
					result = result - self.term()?;
				}
				_ => break invalid("expected * or /")
			}
		}
	}
}

fn main() -> std::io::Result<()> {
	loop {
		let mut line = String::new();
		println!("Enter an expression");
		stdin().read_line(&mut line)?;
		let mut lexer = Lexer::new(&line);
		let mut interpreter = Interpreter::new(&mut lexer)?;
		println!("Result = {}", interpreter.expr()?.to_string());
	}
}
