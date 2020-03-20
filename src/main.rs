use std::io::*;
use std::str::*;
use std::format;

#[derive(Debug, Eq, PartialEq)]
enum Token {
	Integer { value: u32 },
	Plus,
	Eof
}

struct Lexer<'a> {
	text: Chars<'a>
}

fn invalid<A>(msg: &str) -> Result<A> {
	Err(Error::new(ErrorKind::InvalidData, msg))
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
	eat(&lexer.next_token()?, &Token::Plus)?;
	let right = integer(&lexer.next_token()?)?;

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
