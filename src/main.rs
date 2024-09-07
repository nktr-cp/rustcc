mod error;
mod lexer;
mod parser;
mod gen;

use std::env;

fn main() {
	let args: Vec<String> = env::args().collect();
	if args.len() != 2 {
		error::error("引数の個数が正しくありません");
	}

	let input = &args[1];
	let tokens = lexer::tokenize(input);
	let mut parser = parser::Parser::new(tokens);
	let code = parser.program().expect("構文解析に失敗しました");

	println!(".intel_syntax noprefix");
	println!(".global main");
	println!("main:");

	// prologue
	// 26文字の変数分の領域を確保
	println!("  push rbp");
	println!("  mov rbp, rsp");
	println!("  sub rsp, 208");

	for node in code.iter() {
		gen::gen(node);
		println!("  pop rax");
	}
	
	// epilogue
	println!("  mov rsp, rbp");
	println!("  pop rbp");
	println!("  ret");
}
