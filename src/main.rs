mod error;
mod gen;
mod lexer;
mod parser;

use std::env;
use crate::parser::NodeKind;

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

    let mut id = 0;
    for (i, node) in code.iter().enumerate() {
        gen::gen(node, &mut id);
				if matches!(node.kind, NodeKind::GVarDef(_)) || i == code.len() - 1 {
					continue;
				}
            println!("  pop rax");
    }
}
