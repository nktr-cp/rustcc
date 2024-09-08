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
    // pub lvar: Option<i32>,
    pub lvar: Option<LVar>,
}

// debug
// impl Node {
// 	pub fn print(&self) {
// 		match self.kind {
// 			NodeKind::Add => {
// 				println!("Add");
// 				self.lhs.as_ref().unwrap().print();
// 				self.rhs.as_ref().unwrap().print();
// 			}
// 			NodeKind::Sub => {
// 				println!("Sub");
// 				self.lhs.as_ref().unwrap().print();
// 				self.rhs.as_ref().unwrap().print();
// 			}
// 			NodeKind::Mul => {
// 				println!("Mul");
// 				self.lhs.as_ref().unwrap().print();
// 				self.rhs.as_ref().unwrap().print();
// 			}
// 			NodeKind::Div => {
// 				println!("Div");
// 				self.lhs.as_ref().unwrap().print();
// 				self.rhs.as_ref().unwrap().print();
// 			}
// 			NodeKind::Num => {
// 				println!("Num: {}", self.val.unwrap());
// 			}
// 			NodeKind::For => {
// 				println!("For");
// 				self.lhs.as_ref().unwrap().print();
// 				self.rhs.as_ref().unwrap().print();
// 			}
// 			NodeKind::While => {
// 				println!("While");
// 				self.lhs.as_ref().unwrap().print();
// 				self.rhs.as_ref().unwrap().print();
// 			}
// 			NodeKind::If => {
// 				println!("If");
// 				self.lhs.as_ref().unwrap().print();
// 				self.rhs.as_ref().unwrap().print();
// 			}
// 			NodeKind::Else => {
// 				println!("Else");
// 				self.lhs.as_ref().unwrap().print();
// 				self.rhs.as_ref().unwrap().print();
// 			}
// 			NodeKind::Eq => {
// 				println!("Eq");
// 				self.lhs.as_ref().unwrap().print();
// 				self.rhs.as_ref().unwrap().print();
// 			}
// 			NodeKind::Nq => {
// 				println!("Nq");
// 				self.lhs.as_ref().unwrap().print();
// 				self.rhs.as_ref().unwrap().print();
// 			}
// 			NodeKind::Lt => {
// 				println!("Lt");
// 				self.lhs.as_ref().unwrap().print();
// 				self.rhs.as_ref().unwrap().print();
// 			}
// 			NodeKind::Le => {
// 				println!("Le");
// 				self.lhs.as_ref().unwrap().print();
// 				self.rhs.as_ref().unwrap().print();
// 			}
// 			NodeKind::Gt => {
// 				println!("Gt");
// 				self.lhs.as_ref().unwrap().print();
// 				self.rhs.as_ref().unwrap().print();
// 			}
// 			NodeKind::Ge => {
// 				println!("Ge");
// 				self.lhs.as_ref().unwrap().print();
// 				self.rhs.as_ref().unwrap().print();
// 			}
// 			NodeKind::Assign => {
// 				println!("Assign");
// 				self.lhs.as_ref().unwrap().print();
// 				self.rhs.as_ref().unwrap().print();
// 			}
// 			NodeKind::Lvar => {
// 				println!("Lvar: {}", self.lvar.as_ref().unwrap().name);
// 			}
// 			NodeKind::Return => {
// 				println!("Return");
// 				self.lhs.as_ref().unwrap().print();
// 			}
// 		}
// 	}
// }

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
    pub locals: HashMap<String, LVar>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens,
            pos: 0,
            locals: HashMap::new(),
        }
    }

    fn find_or_create_lvar(&mut self, name: &str) -> LVar {
        if let Some(lvar) = self.locals.get(name) {
            return lvar.clone();
        }
        let lvar = LVar {
            name: name.to_string(),
            offset: (self.locals.len() + 1) * 8,
        };
        self.locals.insert(name.to_string(), lvar.clone());
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
            nodes.push(self.stmt()?);
        }
        return Ok(nodes);
    }

    fn stmt(&mut self) -> Result<Node, String> {
        let node: Node;
				if self.consume("return") {
					node = Node {
							kind: NodeKind::Return,
							lhs: Some(Box::new(self.expr()?)),
							rhs: None,
							val: None,
							lvar: None,
					};
					self.expect(";")?;
        } else if self.consume("for") {
					let mut init = Node {
							kind: NodeKind::Num,
							lhs: None,
							rhs: None,
							val: Some(0),
							lvar: None,
					};
					let mut cond = Node {
							kind: NodeKind::Num,
							lhs: None,
							rhs: None,
							val: Some(1), // default condition is true
							lvar: None,
					};
					let mut inc = Node {
							kind: NodeKind::Num,
							lhs: None,
							rhs: None,
							val: Some(0),
							lvar: None,
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
									})),
									val: None,
									lvar: None,
							})),
							val: None,
							lvar: None,
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
								})),
								val: None,
								lvar: None,
						};
					} else {
						node = Node {
								kind: NodeKind::If,
								lhs: Some(Box::new(cond)),
								rhs: Some(Box::new(then)),
								val: None,
								lvar: None,
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
                };
            } else if self.consume("!=") {
                node = Node {
                    kind: NodeKind::Nq,
                    lhs: Some(Box::new(node)),
                    rhs: Some(Box::new(self.relational()?)),
                    val: None,
                    lvar: None,
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
                };
            } else if self.consume("<=") {
                node = Node {
                    kind: NodeKind::Le,
                    lhs: Some(Box::new(node)),
                    rhs: Some(Box::new(self.add()?)),
                    val: None,
                    lvar: None,
                };
            } else if self.consume(">") {
                node = Node {
                    kind: NodeKind::Gt,
                    lhs: Some(Box::new(node)),
                    rhs: Some(Box::new(self.add()?)),
                    val: None,
                    lvar: None,
                };
            } else if self.consume(">=") {
                node = Node {
                    kind: NodeKind::Ge,
                    lhs: Some(Box::new(node)),
                    rhs: Some(Box::new(self.add()?)),
                    val: None,
                    lvar: None,
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
                };
            } else if self.consume("-") {
                node = Node {
                    kind: NodeKind::Sub,
                    lhs: Some(Box::new(node)),
                    rhs: Some(Box::new(self.mul()?)),
                    val: None,
                    lvar: None,
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
                };
            } else if self.consume("/") {
                node = Node {
                    kind: NodeKind::Div,
                    lhs: Some(Box::new(node)),
                    rhs: Some(Box::new(self.unary()?)),
                    val: None,
                    lvar: None,
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
                })),
                rhs: Some(Box::new(self.primary()?)),
                val: None,
                lvar: None,
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
            Ok(Node {
                kind: NodeKind::Lvar,
                lhs: None,
                rhs: None,
                val: None,
                lvar: Some(lvar),
            })
        } else {
            Ok(Node {
                kind: NodeKind::Num,
                lhs: None,
                rhs: None,
                val: Some(self.expect_number()?),
                lvar: None,
            })
        }
    }
}
