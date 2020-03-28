use std::io::*;
use std::format;

mod ast;
use ast::*;

mod lexer;
mod parser;
mod interpreter;

fn main() -> std::io::Result<()> {
	loop {
		let mut line = String::new();
		println!("Enter an expression");
		stdin().read_line(&mut line)?;
		let lexer = lexer::Lexer::new(&line);
		let mut parser = parser::Parser::new(lexer)?;
		let ast = parser.parse()?;
		let mut global_scope = interpreter::Scope::new();
		interpreter::interpret(&ast, &mut global_scope);
		println!("{:?}", global_scope);
	}
}
