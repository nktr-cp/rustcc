mod error;
mod tokenizer;
mod parser;
mod compiler;

use std::env;

fn main() {
	let args: Vec<String> = env::args().collect();
	if args.len() != 2 {
		error::error("引数の個数が正しくありません");
	}

	let input = &args[1];
	let tokens = tokenizer::tokenize(input);
	// let mut parser = parser::Parser::new(tokens.clone());
	// let node = parser.expr().expect("パースに失敗しました");

	// node.print();

	compiler::compile(&tokens);
}
