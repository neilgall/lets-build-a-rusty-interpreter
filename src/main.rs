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
enum UnaryOp {
	Plus,
	Minus
}

#[derive(Debug, Eq, PartialEq)]
enum BinaryOp {
	Plus,
	Minus,
	Multiply,
	Divide
}

impl fmt::Display for BinaryOp {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			BinaryOp::Plus => write!(f, "+"),
			BinaryOp::Minus => write!(f, "-"),
			BinaryOp::Multiply => write!(f, "*"),
			BinaryOp::Divide => write!(f, "/")
		}
	}
}

#[derive(Debug, Eq, PartialEq)]
enum AST {
	UnaryOp { token: Token, op: UnaryOp, expr: Box<AST> },
	BinaryOp { token: Token, lhs: Box<AST>, op: BinaryOp, rhs: Box<AST> },
	Number { token: Token, value: u32 }
}

impl AST {
	fn unary_op(token: Token, expr: AST) -> Self {
		let op = match token {
			Token::Plus => UnaryOp::Plus,
			Token::Minus => UnaryOp::Minus,
			_ => panic!("invalid unary op {:?}", token)
		};
		AST::UnaryOp {
			token,
			op,
			expr: Box::new(expr)
		}
	}

	fn binary_op(lhs: AST, token: Token, rhs: AST) -> Self {
		let op = match token {
			Token::Plus => BinaryOp::Plus,
			Token::Minus => BinaryOp::Minus,
			Token::Multiply => BinaryOp::Multiply,
			Token::Divide => BinaryOp::Divide,
			_ => panic!("invalid binary op {:?}", token)
		};
		AST::BinaryOp {
			token,
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
			Token::Plus | Token::Minus => {
				let token = self.current_token;
				self.advance()?;
				Ok(AST::unary_op(token, self.expr()?))
			}
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

fn interpret(ast: &AST) -> i32 {
	match ast {
		AST::UnaryOp { token: _, op, expr } => {
			let expr_value = interpret(&expr);
			match op {
				UnaryOp::Plus => expr_value,
				UnaryOp::Minus => -expr_value
			}
		}
		AST::BinaryOp { token: _, lhs, op ,rhs } => {
			let lhs_value = interpret(&lhs);
			let rhs_value = interpret(&rhs);
			match op {
				BinaryOp::Plus => lhs_value + rhs_value,
				BinaryOp::Minus => lhs_value - rhs_value,
				BinaryOp::Multiply => lhs_value * rhs_value,
				BinaryOp::Divide => lhs_value / rhs_value,
			}
		}
		AST::Number { token: _, value } => {
			*value as i32
		}
	}
}

fn to_postfix(ast: &AST) -> String {
	match ast {
		AST::Number { token: _, value } => {
			value.to_string()
		}
		AST::UnaryOp { token: _, op, expr } => {
			let expr_value = to_postfix(&expr);
			match op {
				UnaryOp::Plus => expr_value,
				UnaryOp::Minus => format!("{} neg", expr_value)
			}
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
		AST::UnaryOp { token: _, op, expr } => {
			let expr_value = to_s_expr(&expr);
			match op {
				UnaryOp::Plus => expr_value,
				UnaryOp::Minus => format!("(neg {})", expr_value)
			}
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
