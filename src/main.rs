use std::fmt;
use std::io::*;
use std::str::*;
use std::format;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Token {
	Integer { value: u32 },
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
					_ => { 
						invalid(&format!("invalid character '{}'", c))
					}
				}
		}
	}
}

#[derive(Debug, Eq, PartialEq)]
enum AST {
	BinaryOp { token: Token, lhs: Box<AST>, op: Token, rhs: Box<AST> },
	Number { token: Token, value: u32 }
}

impl AST {
	fn binary_op(lhs: AST, op: Token, rhs: AST) -> Self {
		AST::BinaryOp {
			token: op.clone(),
			lhs: Box::new(lhs),
			op,
			rhs: Box::new(rhs)
		}
	}

	fn number(token: Token, value: u32) -> Self {
		AST::Number {
			token: token.clone(),
			value
		}
	}
}

struct Parser<'a> {
	lexer: &'a mut Lexer<'a>,
	current_token: Token
}

impl<'a> Parser<'a> {
	fn new(lexer: &'a mut Lexer<'a>) -> Result<Self> {
		let token = lexer.next_token()?;
		Ok(Parser {
			lexer,
			current_token: token
		})
	}

	fn advance(&mut self) -> Result<()> {
		self.current_token = self.lexer.next_token()?;
		Ok(())
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
			Token::OpenParen => {
				self.advance()?;
				let result = self.expr()?;
				self.eat(&Token::CloseParen)?;
				Ok(result)
			}
			Token::Integer { value } => {
				self.advance()?;
				Ok(AST::number(self.current_token, value))
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
					let op = self.current_token;
					self.advance()?;
					node = AST::binary_op(node, op, self.factor()?);
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
					let op = self.current_token;
					self.advance()?;
					node = AST::binary_op(node, op, self.term()?);
				}
				_ => {
					break Ok(node)
				}
			}
		}
	}

	fn parse(&mut self) -> Result<AST> {
		self.expr()
	}
}

fn interpret(ast: &AST) -> u32 {
	match ast {
		AST::Number { token: _, value } => {
			*value
		}
		AST::BinaryOp { token: _, lhs, op ,rhs } => {
			let lhs_value = interpret(&lhs);
			let rhs_value = interpret(&rhs);
			match op {
				Token::Plus => lhs_value + rhs_value,
				Token::Minus => lhs_value - rhs_value,
				Token::Multiply => lhs_value * rhs_value,
				Token::Divide => lhs_value / rhs_value,
				_ => panic!("unexpected binary operation {:?}", op)
			}
		}
	}
}

fn to_postfix(ast: &AST) -> String {
	match ast {
		AST::Number { token: _, value } => {
			value.to_string()
		}
		AST::BinaryOp { token: _, lhs, op, rhs } => {
			let lhs_value = to_postfix(&lhs);
			let rhs_value = to_postfix(&rhs);
			format!("{} {} {}", lhs_value, rhs_value, op)
		}
	}
}

fn to_s_expr(ast: &AST) -> String {
	match ast {
		AST::Number { token: _, value } => {
			value.to_string()
		}
		AST::BinaryOp { token: _, lhs, op, rhs } => {
			let lhs_value = to_s_expr(&lhs);
			let rhs_value = to_s_expr(&rhs);
			format!("({} {} {})", op, lhs_value, rhs_value)
		}
	}
}

fn main() -> std::io::Result<()> {
	loop {
		let mut line = String::new();
		println!("Enter an expression");
		stdin().read_line(&mut line)?;
		let mut lexer = Lexer::new(&line);
		let mut parser = Parser::new(&mut lexer)?;
		let ast = parser.parse()?;
		println!("postfix: {}", to_postfix(&ast));
		println!("s_expr:  {}", to_s_expr(&ast));
		println!("result:  {}", interpret(&ast));
	}
}
