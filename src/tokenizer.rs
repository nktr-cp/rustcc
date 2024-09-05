use crate::error;

#[derive(Debug, PartialEq, Clone)]
pub enum TokenKind {
	Reserved,
	Num,
	Eof,
}

#[derive(Debug, Clone)]
pub struct Token {
	pub kind: TokenKind,
	pub val: Option<i32>,
	pub str: String,
}

pub fn tokenize(input: &str) -> Vec<Token> {
	let mut tokens = Vec::new();
	let mut chars = input.chars().peekable();

	while let Some(&c) = chars.peek() {
		if c.is_whitespace() {
			chars.next();
			continue;
		}

		if "+-*/()".contains(c) {
			tokens.push(Token {
				kind: TokenKind::Reserved,
				val: None,
				str: c.to_string(),
			});
			chars.next();
			continue;
		}

		if c.is_digit(10) {
			let mut num_str = String::new();
			while let Some(&c) = chars.peek() {
				if !c.is_digit(10) {
					break;
				}
				num_str.push(c);
				chars.next();
			}
			tokens.push(Token {
				kind: TokenKind::Num,
				val: Some(num_str.parse().unwrap()),
				str: num_str,
			});
			continue;
		}

		error::error("トークナイズできません");
	}

	tokens.push(Token {
		kind: TokenKind::Eof,
		val: None,
		str: String::new(),
	});

	tokens
}
