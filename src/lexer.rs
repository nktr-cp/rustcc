use crate::error;

#[derive(Debug, PartialEq, Clone)]
pub enum TokenKind {
    Reserved,
    Ident,
    Return,
    For,
    While,
    If,
    Else,
    Num,
    Sizeof,
    Eof,
    Strlit,
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

        if c == '/' && chars.clone().nth(1) == Some('/') {
            chars.next();
            chars.next();
            while let Some(&c) = chars.peek() {
                if c == '\n' {
                    break;
                }
                chars.next();
            }
            continue;
        }

        if c == '/' && chars.clone().nth(1) == Some('*') {
            chars.next();
            chars.next();
            while let Some(&c) = chars.peek() {
                if c == '*' && chars.clone().nth(1) == Some('/') {
                    chars.next();
                    chars.next();
                    break;
                }
                chars.next();
            }
            continue;
        }

        if c == '"' {
            let mut lit = String::new();
            chars.next();
            while let Some(&c) = chars.peek() {
                if c == '"' {
                    break;
                }
                lit.push(c);
                chars.next();
            }
            chars.next();

            tokens.push(Token {
                kind: TokenKind::Strlit,
                val: None,
                str: lit,
            });
            continue;
        }

        if "+-*/();{},&[]".contains(c) {
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
            match ident.as_str() {
                "return" => {
                    tokens.push(Token {
                        kind: TokenKind::Return,
                        val: None,
                        str: ident,
                    });
                    continue;
                }
                "for" => {
                    tokens.push(Token {
                        kind: TokenKind::For,
                        val: None,
                        str: ident,
                    });
                    continue;
                }
                "while" => {
                    tokens.push(Token {
                        kind: TokenKind::While,
                        val: None,
                        str: ident,
                    });
                    continue;
                }
                "if" => {
                    tokens.push(Token {
                        kind: TokenKind::If,
                        val: None,
                        str: ident,
                    });
                    continue;
                }
                "else" => {
                    tokens.push(Token {
                        kind: TokenKind::Else,
                        val: None,
                        str: ident,
                    });
                    continue;
                }
                "int" => {
                    tokens.push(Token {
                        kind: TokenKind::Reserved,
                        val: None,
                        str: ident,
                    });
                    continue;
                }
                "char" => {
                    tokens.push(Token {
                        kind: TokenKind::Reserved,
                        val: None,
                        str: ident,
                    });
                    continue;
                }
                "sizeof" => {
                    tokens.push(Token {
                        kind: TokenKind::Sizeof,
                        val: None,
                        str: ident,
                    });
                    continue;
                }
                _ => {
                    tokens.push(Token {
                        kind: TokenKind::Ident,
                        val: None,
                        str: ident,
                    });
                    continue;
                }
            }
        }

        error::error(format!("トークナイズできません: {}", c).as_str());
    }

    tokens.push(Token {
        kind: TokenKind::Eof,
        val: None,
        str: String::new(),
    });

    tokens
}
