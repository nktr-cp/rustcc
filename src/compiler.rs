use crate::tokenizer::{Token, TokenKind};
use crate::error;

pub fn compile(tokens: &[Token]) {
	let mut iter = tokens.iter().peekable();

	println!(".intel_syntax noprefix");
	println!(".global main");
	println!("main:");

	match iter.next() {
		Some (token) if token.kind == TokenKind::Num => {
			println!("	mov rax, {}", token.val.unwrap());
		}
		_ => error::error("式の最初は数値である必要があります"),
	}

	while let Some(token) = iter.next() {
		match token.kind {
			TokenKind::Reserved if token.str == "+" => {
				if let Some(next_token) = iter.next() {
					if next_token.kind == TokenKind::Num {
						println!("  add rax, {}", next_token.val.unwrap());
					} else {
						error::error("''+''の後には数値が必要です");
					}
				} else {
					error::error("''+''の後には数値が必要です");
				}
			}

			TokenKind::Reserved if token.str == "-" => {
				if let Some(next_token) = iter.next() {
					if next_token.kind == TokenKind::Num {
						println!("  sub rax, {}", next_token.val.unwrap());
					} else {
						error::error("''-''の後には数値が必要です");
					}
				} else {
					error::error("''-''の後には数値が必要です");
				}
			}

			TokenKind::Eof => break,

			_ => error::error("予期しないトークンです"),
		}
	}

	println!("	ret");
}
