use crate::lexer::{Token, TokenKind};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOpKind {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOpKind {
    Ref,
    Deref,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ComparisonOpKind {
    Eq,
    Nq,
    Lt,
    Le,
    Gt,
    Ge,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LVar {
    pub name: String,
    pub offset: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub enum NodeKind {
    BinaryOp(BinaryOpKind),              // Binaty operations: +, -, *, /
    UnaryOp(UnaryOpKind),                // Unary operations: &, *
    Num(i32),                            // Numeric literals
    LVar(LVar),                          // Local variable
    Assign,                              // Assignment
    Return,                              // Return statement
    Block(Vec<Node>),                    // Block of statements
    Fncall(LVar, Vec<Node>),             // Function call
    Fndef(String, Vec<Node>, Vec<LVar>), // Function definition
    For,
    While,
    If,
    Else,
    Comparison(ComparisonOpKind),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Node {
    pub kind: NodeKind,
    pub lhs: Option<Box<Node>>,
    pub rhs: Option<Box<Node>>,
}

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
    fn_idx: usize,
    pub locals: Vec<HashMap<String, LVar>>, // function name -> local variables
    pub functions: Vec<String>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens,
            pos: 0,
            fn_idx: 0, // indicates the index of the current function
            locals: Vec::new(),
            functions: Vec::new(),
        }
    }

    fn find_or_create_lvar(&mut self, name: &str) -> LVar {
        if let Some(lvar) = self.locals[self.fn_idx].get(name) {
            return lvar.clone();
        }
        let lvar = LVar {
            name: name.to_string(),
            offset: self.locals[self.fn_idx].len() * 8 + 8,
        };
        self.locals[self.fn_idx].insert(name.to_string(), lvar.clone());
        lvar
    }

    fn consume(&mut self, op: &str) -> bool {
        if self.tokens[self.pos].str != op.to_string() {
            return false;
        }
        self.pos += 1;
        true
    }

    fn expect(&mut self, op: &str) -> Result<(), String> {
        if !self.consume(op) {
            return Err(format!(
                "'{}'が期待されますが、'{}'でした",
                op, self.tokens[self.pos].str
            ));
        }
        Ok(())
    }

    fn expect_number(&mut self) -> Result<i32, String> {
        if self.tokens[self.pos].kind != TokenKind::Num {
            return Err(format!(
                "数が期待されますが、'{}'でした",
                self.tokens[self.pos].str
            ));
        }
        let val = self.tokens[self.pos].val.unwrap();
        self.pos += 1;
        Ok(val)
    }

    fn at_eof(&self) -> bool {
        self.tokens[self.pos].kind == TokenKind::Eof
    }

    pub fn program(&mut self) -> Result<Vec<Node>, String> {
        let mut nodes = Vec::new();
        while !self.at_eof() {
            self.locals.push(HashMap::new());
            nodes.push(self.function()?);
            self.fn_idx += 1;
        }
        return Ok(nodes);
    }

    fn function(&mut self) -> Result<Node, String> {
        self.expect("int")?;
        let name = self.tokens[self.pos].str.clone();
        self.pos += 1;
        self.expect("(")?;

        let mut params = Vec::new();
        if !self.consume(")") {
            params = self.paramlist()?;
            self.expect(")")?;
        }

        if self.functions.contains(&name) {
            return Err(format!("関数{}は既に定義されています", name));
        } else {
            self.functions.push(name.clone());
        }

        let body = self.stmt()?;

        Ok(Node {
            kind: NodeKind::Fndef(
                name,
                params,
                self.locals[self.fn_idx].values().cloned().collect(),
            ),
            lhs: None,
            rhs: Some(Box::new(body)),
        })
    }

    fn paramlist(&mut self) -> Result<Vec<Node>, String> {
        let mut params = Vec::new();

        loop {
            self.expect("int")?;
            let lvar = self.find_or_create_lvar(&self.tokens[self.pos].str.clone());
            params.push(Node {
                kind: NodeKind::LVar(lvar),
                lhs: None,
                rhs: None,
            });
            self.pos += 1;
            if !self.consume(",") {
                break;
            }
        }
        Ok(params)
    }

    fn stmt(&mut self) -> Result<Node, String> {
        let node: Node;
        if self.consume("{") {
            let mut stmts = Vec::new();
            loop {
                if self.consume("}") {
                    break;
                }
                stmts.push(self.stmt()?);
            }
            node = Node {
                kind: NodeKind::Block(stmts),
                lhs: None,
                rhs: None,
            }
        } else if self.consume("return") {
            node = Node {
                kind: NodeKind::Return,
                lhs: Some(Box::new(self.expr()?)),
                rhs: None,
            };
            self.expect(";")?;
        } else if self.consume("for") {
            let mut init = Node {
                kind: NodeKind::Num(0),
                lhs: None,
                rhs: None,
            };
            let mut cond = Node {
                kind: NodeKind::Num(1), // default should be true
                lhs: None,
                rhs: None,
            };
            let mut inc = Node {
                kind: NodeKind::Num(0),
                lhs: None,
                rhs: None,
            };
            self.expect("(")?;
            if !self.consume(";") {
                init = self.expr()?;
                self.expect(";")?;
            }
            if !self.consume(";") {
                cond = self.expr()?;
                self.expect(";")?;
            }
            if !self.consume(")") {
                inc = self.expr()?;
                self.expect(")")?;
            }
            node = Node {
                kind: NodeKind::For,
                lhs: Some(Box::new(init)),
                rhs: Some(Box::new(Node {
                    kind: NodeKind::For,
                    lhs: Some(Box::new(cond)),
                    rhs: Some(Box::new(Node {
                        kind: NodeKind::For,
                        lhs: Some(Box::new(inc)),
                        rhs: Some(Box::new(self.stmt()?)),
                    })),
                })),
            };
        } else if self.consume("while") {
            self.expect("(")?;
            let cond = self.expr()?;
            self.expect(")")?;
            node = Node {
                kind: NodeKind::While,
                lhs: Some(Box::new(cond)),
                rhs: Some(Box::new(self.stmt()?)),
            };
        } else if self.consume("if") {
            self.expect("(")?;
            let cond = self.expr()?;
            self.expect(")")?;
            let then = self.stmt()?;
            if self.consume("else") {
                let els = self.stmt()?;
                node = Node {
                    kind: NodeKind::If,
                    lhs: Some(Box::new(cond)),
                    rhs: Some(Box::new(Node {
                        kind: NodeKind::Else,
                        lhs: Some(Box::new(then)),
                        rhs: Some(Box::new(els)),
                    })),
                };
            } else {
                node = Node {
                    kind: NodeKind::If,
                    lhs: Some(Box::new(cond)),
                    rhs: Some(Box::new(then)),
                };
            }
        } else if self.consume("int") {
            node = self.decl()?;
            self.expect(";")?;
        } else {
            node = self.expr()?;
            self.expect(";")?;
        }
        Ok(node)
    }

    fn decl(&mut self) -> Result<Node, String> {
        let node;
        let lvar = self.find_or_create_lvar(&self.tokens[self.pos].str.clone());
        self.pos += 1;
        if self.consume("=") {
            node = Node {
                kind: NodeKind::Assign,
                lhs: Some(Box::new(Node {
                    kind: NodeKind::LVar(lvar.clone()),
                    lhs: None,
                    rhs: None,
                })),
                rhs: Some(Box::new(self.expr()?)),
            };
        } else {
            node = Node {
                kind: NodeKind::LVar(lvar.clone()),
                lhs: None,
                rhs: None,
            };
        }
        Ok(node)
    }

    fn expr(&mut self) -> Result<Node, String> {
        return self.assign();
    }

    fn assign(&mut self) -> Result<Node, String> {
        let mut node = self.equality()?;

        if self.consume("=") {
            node = Node {
                kind: NodeKind::Assign,
                lhs: Some(Box::new(node)),
                rhs: Some(Box::new(self.assign()?)),
            };
        }

        return Ok(node);
    }

    fn equality(&mut self) -> Result<Node, String> {
        let mut node = self.relational()?;

        loop {
            if self.consume("==") {
                node = Node {
                    kind: NodeKind::Comparison(ComparisonOpKind::Eq),
                    lhs: Some(Box::new(node)),
                    rhs: Some(Box::new(self.relational()?)),
                };
            } else if self.consume("!=") {
                node = Node {
                    kind: NodeKind::Comparison(ComparisonOpKind::Nq),
                    lhs: Some(Box::new(node)),
                    rhs: Some(Box::new(self.relational()?)),
                };
            } else {
                return Ok(node);
            }
        }
    }

    fn relational(&mut self) -> Result<Node, String> {
        let mut node = self.add()?;

        loop {
            if self.consume("<") {
                node = Node {
                    kind: NodeKind::Comparison(ComparisonOpKind::Lt),
                    lhs: Some(Box::new(node)),
                    rhs: Some(Box::new(self.add()?)),
                };
            } else if self.consume("<=") {
                node = Node {
                    kind: NodeKind::Comparison(ComparisonOpKind::Le),
                    lhs: Some(Box::new(node)),
                    rhs: Some(Box::new(self.add()?)),
                };
            } else if self.consume(">") {
                node = Node {
                    kind: NodeKind::Comparison(ComparisonOpKind::Gt),
                    lhs: Some(Box::new(node)),
                    rhs: Some(Box::new(self.add()?)),
                };
            } else if self.consume(">=") {
                node = Node {
                    kind: NodeKind::Comparison(ComparisonOpKind::Ge),
                    lhs: Some(Box::new(node)),
                    rhs: Some(Box::new(self.add()?)),
                };
            } else {
                return Ok(node);
            }
        }
    }

    fn add(&mut self) -> Result<Node, String> {
        let mut node = self.mul()?;

        loop {
            if self.consume("+") {
                node = Node {
                    kind: NodeKind::BinaryOp(BinaryOpKind::Add),
                    lhs: Some(Box::new(node)),
                    rhs: Some(Box::new(self.mul()?)),
                };
            } else if self.consume("-") {
                node = Node {
                    kind: NodeKind::BinaryOp(BinaryOpKind::Sub),
                    lhs: Some(Box::new(node)),
                    rhs: Some(Box::new(self.mul()?)),
                };
            } else {
                return Ok(node);
            }
        }
    }

    fn mul(&mut self) -> Result<Node, String> {
        let mut node = self.unary()?;

        loop {
            if self.consume("*") {
                node = Node {
                    kind: NodeKind::BinaryOp(BinaryOpKind::Mul),
                    lhs: Some(Box::new(node)),
                    rhs: Some(Box::new(self.unary()?)),
                };
            } else if self.consume("/") {
                node = Node {
                    kind: NodeKind::BinaryOp(BinaryOpKind::Div),
                    lhs: Some(Box::new(node)),
                    rhs: Some(Box::new(self.unary()?)),
                };
            } else {
                return Ok(node);
            }
        }
    }

    fn unary(&mut self) -> Result<Node, String> {
        if self.consume("+") {
            return self.primary();
        } else if self.consume("-") {
            // 0 - x として扱う
            return Ok(Node {
                kind: NodeKind::BinaryOp(BinaryOpKind::Sub),
                lhs: Some(Box::new(Node {
                    kind: NodeKind::Num(0),
                    lhs: None,
                    rhs: None,
                })),
                rhs: Some(Box::new(self.primary()?)),
            });
        } else if self.consume("&") {
            return Ok(Node {
                kind: NodeKind::UnaryOp(UnaryOpKind::Ref),
                lhs: Some(Box::new(self.unary()?)),
                rhs: None,
            });
        } else if self.consume("*") {
            return Ok(Node {
                kind: NodeKind::UnaryOp(UnaryOpKind::Deref),
                lhs: Some(Box::new(self.unary()?)),
                rhs: None,
            });
        } else {
            return self.primary();
        }
    }

    fn primary(&mut self) -> Result<Node, String> {
        if self.consume("(") {
            let node = self.expr()?;
            self.expect(")")?;
            Ok(node)
        } else if self.tokens[self.pos].kind == TokenKind::Ident {
            let lvar = self.find_or_create_lvar(&self.tokens[self.pos].str.clone());
            self.pos += 1;
            if self.consume("(") {
                let mut args = Vec::new();
                if self.consume(")") {
                    Ok(Node {
                        kind: NodeKind::Fncall(lvar, args),
                        lhs: None,
                        rhs: None,
                    })
                } else {
                    args = self.arglist()?;
                    Ok(Node {
                        kind: NodeKind::Fncall(lvar, args),
                        lhs: None,
                        rhs: None,
                    })
                }
            } else {
                Ok(Node {
                    kind: NodeKind::LVar(lvar),
                    lhs: None,
                    rhs: None,
                })
            }
        } else {
            Ok(Node {
                kind: NodeKind::Num(self.expect_number()?),
                lhs: None,
                rhs: None,
            })
        }
    }

    fn arglist(&mut self) -> Result<Vec<Node>, String> {
        let mut args = Vec::new();
        args.push(self.expr()?);

        loop {
            if self.consume(",") {
                args.push(self.expr()?);
            } else {
                break;
            }
        }

        self.expect(")")?;
        Ok(args)
    }
}
