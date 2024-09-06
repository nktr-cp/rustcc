mod error;
mod tokenizer;
mod parser;
mod gen;

use std::env;

fn main() {
	let args: Vec<String> = env::args().collect();
	if args.len() != 2 {
		error::error("引数の個数が正しくありません");
	}

	let input = &args[1];
	let tokens = tokenizer::tokenize(input);
	let mut parser = parser::Parser::new(tokens);
	let node = parser.expr().expect("パースに失敗しました");

	println!(".intel_syntax noprefix");
	println!(".global main");
	println!("main:");

	gen::gen(&node);
	
	println!("  pop rax");
	println!("  ret");
}
