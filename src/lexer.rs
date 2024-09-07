use crate::error;

#[derive(Debug, PartialEq, Clone)]
pub enum TokenKind {
    Reserved,
    Ident,
    Return,
    Num,
    Eof,
}

#[derive(Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub val: Option<i32>,
    pub str: String,
}

fn is_alnum(c: char) -> bool {
    c.is_ascii_alphabetic() || c.is_ascii_digit() || c == '_'
}

pub fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(&c) = chars.peek() {
        if c.is_whitespace() {
            chars.next();
            continue;
        }

        if "+-*/();".contains(c) {
            tokens.push(Token {
                kind: TokenKind::Reserved,
                val: None,
                str: c.to_string(),
            });
            chars.next();
            continue;
        }

        if c == '=' || c == '!' || c == '<' || c == '>' {
            let mut op = String::new();
            op.push(c);
            chars.next();

            if let Some(&next_c) = chars.peek() {
                if next_c == '=' {
                    op.push(next_c);
                    chars.next();
                }
            }

            tokens.push(Token {
                kind: TokenKind::Reserved,
                val: None,
                str: op,
            });
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

        if is_alnum(c) {
            let mut ident = String::new();
            while let Some(&c) = chars.peek() {
                if !is_alnum(c) {
                    break;
                }
                ident.push(c);
                chars.next();
            }
            if ident == "return" {
                tokens.push(Token {
                    kind: TokenKind::Return,
                    val: None,
                    str: ident,
                });
                continue;
            } else {
                tokens.push(Token {
                    kind: TokenKind::Ident,
                    val: None,
                    str: ident,
                });
                continue;
            }
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
