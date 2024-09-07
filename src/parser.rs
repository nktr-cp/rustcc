use std::collections::HashMap;
use crate::lexer::{Token, TokenKind};

#[derive(Debug, Clone, PartialEq)]
pub enum NodeKind {
	Add,
	Sub,
	Mul,
	Div,
	Assign,
	Lvar,
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
// 		}
// 	}
// }

pub struct Parser {
	tokens: Vec<Token>,
	pos: usize,
	locals: HashMap<String, LVar>,
}

impl Parser {
	pub fn new(tokens: Vec<Token>) -> Self {
		Parser { tokens, pos: 0, locals: HashMap::new() }
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
		if self.tokens[self.pos].kind != TokenKind::Reserved || self.tokens[self.pos].str != op.to_string() {
			return false;
		}
		self.pos += 1;
		true
	}

	fn expect(&mut self, op: &str) -> Result<(), String> {
		if !self.consume(op) {
			return Err(format!("'{}'が期待されますが、'{}'でした", op, self.tokens[self.pos].str));
		}
		Ok(())
	}

	fn expect_number(&mut self) -> Result<i32, String> {
		if self.tokens[self.pos].kind != TokenKind::Num {
			return Err(format!("数が期待されますが、'{}'でした", self.tokens[self.pos].str));
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
		let node = self.expr()?;
		self.expect(";")?;
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
