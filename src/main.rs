use std::io::*;
use std::str::*;
use std::format;

enum Token {
	Integer { value: u32 },
	Plus,
	Eof
}

struct Lexer<'a> {
	text: Chars<'a>
}

impl<'a> Lexer<'a> {
	fn new(text: &'a str) -> Self {
		Lexer {
			text: text.chars()
		}
	}

	fn next_token(&mut self) -> Result<Token> {
		match self.text.next() {
			None => 
				Ok(Token::Eof),
			
			Some(c) =>
				if c == '+' {
					Ok(Token::Plus)
				} else if c.is_digit(10) {
					Ok(Token::Integer { value: c.to_digit(10).unwrap() })
				} else {
					Err(Error::new(ErrorKind::InvalidData, format!("invalid character '{}'", c)))
				}
		}
	}
}

fn interpret(text: &str) -> Result<u32> {
	let mut lexer = Lexer::new(&text);
	let left = match lexer.next_token()? {
		Token::Integer { value } => Ok(value),
		_                        => Err(Error::new(ErrorKind::InvalidData, "expected an integer"))
	}?;
	match lexer.next_token()? {
		Token::Plus => Ok(()),
		_           => Err(Error::new(ErrorKind::InvalidData, "expected '+'"))
	}?;
	let right = match lexer.next_token()? {
		Token::Integer { value } => Ok(value),
		_                        => Err(Error::new(ErrorKind::InvalidData, "expected an integer"))
	}?;

	Ok(left + right)
}

fn main() -> std::io::Result<()> {
	loop {
		let mut line = String::new();
		println!("Enter an expression");
		stdin().read_line(&mut line)?;
		println!("Result = {}", interpret(&line)?.to_string());
	}
}
