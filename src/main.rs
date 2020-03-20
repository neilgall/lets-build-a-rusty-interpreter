use std::io::*;
use std::str::*;
use std::format;

#[derive(Debug, Eq, PartialEq)]
enum Token {
	Integer { value: u32 },
	Plus,
	Minus,
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

	fn next_token(&mut self) -> Result<Token> {
		while self.current.map_or(false, |c| c.is_whitespace()) {
			self.advance();
		}

		match self.current {
			None => 
				Ok(Token::Eof),
			
			Some(c) =>
				if c == '+' {
					self.advance();
					Ok(Token::Plus)
				} else if c == '-' {
					self.advance();
					Ok(Token::Minus)
				} else if c.is_digit(10) {
					self.advance();
					Ok(Token::Integer { value: c.to_digit(10).unwrap() })
				} else {
					invalid(&format!("invalid character '{}'", c))
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
