use crate::lexer::{Token, TokenKind};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum NodeKind {
    Add,
    Sub,
    Mul,
    Div,
    Assign,
    Lvar,
    Return,
    For,
    While,
    If,
    Else,
    Eq,
    Nq,
    Lt,
    Le,
    Gt,
    Ge,
    Num,
    Block,
    Fncall,
    Fndef,
    Ref,
    Deref,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LVar {
    pub name: String,
    pub offset: usize,
}

#[derive(Debug, Clone)]
pub struct Node {
    pub kind: NodeKind,
    pub lhs: Option<Box<Node>>,
    pub rhs: Option<Box<Node>>,
    pub val: Option<i32>,
    pub lvar: Option<LVar>,
    pub params: Option<Vec<Node>>,
    pub locals: Option<Vec<LVar>>, // 関数単位でのローカル変数
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
            kind: NodeKind::Fndef,
            lhs: None,
            rhs: Some(Box::new(body)),
            val: None,
            lvar: Some(LVar {
                name: name.clone(),
                offset: 0,
            }),
            params: Some(params),
            locals: Some(self.locals[self.fn_idx].values().cloned().collect()),
        })
    }

    fn paramlist(&mut self) -> Result<Vec<Node>, String> {
        let mut params = Vec::new();

        loop {
            let lvar = self.find_or_create_lvar(&self.tokens[self.pos].str.clone());
            params.push(Node {
                kind: NodeKind::Lvar,
                lhs: None,
                rhs: None,
                val: None,
                lvar: Some(lvar),
                params: None,
                locals: None,
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
                kind: NodeKind::Block,
                lhs: None,
                rhs: None,
                val: None,
                lvar: None,
                params: Some(stmts),
                locals: None,
            }
        } else if self.consume("return") {
            node = Node {
                kind: NodeKind::Return,
                lhs: Some(Box::new(self.expr()?)),
                rhs: None,
                val: None,
                lvar: None,
                params: None,
                locals: None,
            };
            self.expect(";")?;
        } else if self.consume("for") {
            let mut init = Node {
                kind: NodeKind::Num,
                lhs: None,
                rhs: None,
                val: Some(0),
                lvar: None,
                params: None,
                locals: None,
            };
            let mut cond = Node {
                kind: NodeKind::Num,
                lhs: None,
                rhs: None,
                val: Some(1), // default condition is true
                lvar: None,
                params: None,
                locals: None,
            };
            let mut inc = Node {
                kind: NodeKind::Num,
                lhs: None,
                rhs: None,
                val: Some(0),
                lvar: None,
                params: None,
                locals: None,
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
                        val: None,
                        lvar: None,
                        params: None,
                        locals: None,
                    })),
                    val: None,
                    lvar: None,
                    params: None,
                    locals: None,
                })),
                val: None,
                lvar: None,
                params: None,
                locals: None,
            };
        } else if self.consume("while") {
            self.expect("(")?;
            let cond = self.expr()?;
            self.expect(")")?;
            node = Node {
                kind: NodeKind::While,
                lhs: Some(Box::new(cond)),
                rhs: Some(Box::new(self.stmt()?)),
                val: None,
                lvar: None,
                params: None,
                locals: None,
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
                        val: None,
                        lvar: None,
                        params: None,

                        locals: None,
                    })),
                    val: None,
                    lvar: None,
                    params: None,
                    locals: None,
                };
            } else {
                node = Node {
                    kind: NodeKind::If,
                    lhs: Some(Box::new(cond)),
                    rhs: Some(Box::new(then)),
                    val: None,
                    lvar: None,
                    params: None,
                    locals: None,
                };
            }
        } else {
            node = self.expr()?;
            self.expect(";")?;
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
                val: None,
                lvar: None,
                params: None,
                locals: None,
            };
        }

        return Ok(node);
    }

    fn equality(&mut self) -> Result<Node, String> {
        let mut node = self.relational()?;

        loop {
            if self.consume("==") {
                node = Node {
                    kind: NodeKind::Eq,
                    lhs: Some(Box::new(node)),
                    rhs: Some(Box::new(self.relational()?)),
                    val: None,
                    lvar: None,
                    params: None,
                    locals: None,
                };
            } else if self.consume("!=") {
                node = Node {
                    kind: NodeKind::Nq,
                    lhs: Some(Box::new(node)),
                    rhs: Some(Box::new(self.relational()?)),
                    val: None,
                    lvar: None,
                    params: None,
                    locals: None,
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
                    kind: NodeKind::Lt,
                    lhs: Some(Box::new(node)),
                    rhs: Some(Box::new(self.add()?)),
                    val: None,
                    lvar: None,
                    params: None,
                    locals: None,
                };
            } else if self.consume("<=") {
                node = Node {
                    kind: NodeKind::Le,
                    lhs: Some(Box::new(node)),
                    rhs: Some(Box::new(self.add()?)),
                    val: None,
                    lvar: None,
                    params: None,
                    locals: None,
                };
            } else if self.consume(">") {
                node = Node {
                    kind: NodeKind::Gt,
                    lhs: Some(Box::new(node)),
                    rhs: Some(Box::new(self.add()?)),
                    val: None,
                    lvar: None,
                    params: None,
                    locals: None,
                };
            } else if self.consume(">=") {
                node = Node {
                    kind: NodeKind::Ge,
                    lhs: Some(Box::new(node)),
                    rhs: Some(Box::new(self.add()?)),
                    val: None,
                    lvar: None,
                    params: None,
                    locals: None,
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
                    kind: NodeKind::Add,
                    lhs: Some(Box::new(node)),
                    rhs: Some(Box::new(self.mul()?)),
                    val: None,
                    lvar: None,
                    params: None,
                    locals: None,
                };
            } else if self.consume("-") {
                node = Node {
                    kind: NodeKind::Sub,
                    lhs: Some(Box::new(node)),
                    rhs: Some(Box::new(self.mul()?)),
                    val: None,
                    lvar: None,
                    params: None,
                    locals: None,
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
                    kind: NodeKind::Mul,
                    lhs: Some(Box::new(node)),
                    rhs: Some(Box::new(self.unary()?)),
                    val: None,
                    lvar: None,
                    params: None,
                    locals: None,
                };
            } else if self.consume("/") {
                node = Node {
                    kind: NodeKind::Div,
                    lhs: Some(Box::new(node)),
                    rhs: Some(Box::new(self.unary()?)),
                    val: None,
                    lvar: None,
                    params: None,
                    locals: None,
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
                kind: NodeKind::Sub,
                lhs: Some(Box::new(Node {
                    kind: NodeKind::Num,
                    lhs: None,
                    rhs: None,
                    val: Some(0),
                    lvar: None,
                    params: None,
                    locals: None,
                })),
                rhs: Some(Box::new(self.primary()?)),
                val: None,
                lvar: None,
                params: None,
                locals: None,
            });
        } else if self.consume("&") {
            return Ok(Node {
                kind: NodeKind::Ref,
                lhs: Some(Box::new(self.unary()?)),
                rhs: None,
                val: None,
                lvar: None,
                params: None,
                locals: None,
            });
        } else if self.consume("*") {
            return Ok(Node {
                kind: NodeKind::Deref,
                lhs: Some(Box::new(self.unary()?)),
                rhs: None,
                val: None,
                lvar: None,
                params: None,
                locals: None,
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
                        kind: NodeKind::Fncall,
                        lhs: None,
                        rhs: None,
                        val: None,
                        lvar: Some(lvar),
                        params: Some(args),
                        locals: None,
                    })
                } else {
                    args = self.arglist()?;
                    Ok(Node {
                        kind: NodeKind::Fncall,
                        lhs: None,
                        rhs: None,
                        val: None,
                        lvar: Some(lvar),
                        params: Some(args),
                        locals: None,
                    })
                }
            } else {
                Ok(Node {
                    kind: NodeKind::Lvar,
                    lhs: None,
                    rhs: None,
                    val: None,
                    lvar: Some(lvar),
                    params: None,
                    locals: None,
                })
            }
        } else {
            Ok(Node {
                kind: NodeKind::Num,
                lhs: None,
                rhs: None,
                val: Some(self.expect_number()?),
                lvar: None,
                params: None,
                locals: None,
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
