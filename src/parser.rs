use crate::tokenizer::{Token, TokenKind};

#[derive(Debug, Clone)]
pub enum NodeKind {
	Add,
	Sub,
	Mul,
	Div,
	Eq,
	Nq,
	Lt,
	Le,
	Gt,
	Ge,
	Num,
}

#[derive(Debug, Clone)]
pub struct Node {
	pub kind: NodeKind,
	pub lhs: Option<Box<Node>>,
	pub rhs: Option<Box<Node>>,
	pub val: Option<i32>,
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
}

impl Parser {
	pub fn new(tokens: Vec<Token>) -> Self {
		Parser { tokens, pos: 0 }
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

	pub fn expr(&mut self) -> Result<Node, String> {
		return self.equality();
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
				};
			} else if self.consume("!=") {
				node = Node {
					kind: NodeKind::Nq,
					lhs: Some(Box::new(node)),
					rhs: Some(Box::new(self.relational()?)),
					val: None,
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
				};
			} else if self.consume("<=") {
				node = Node {
					kind: NodeKind::Le,
					lhs: Some(Box::new(node)),
					rhs: Some(Box::new(self.add()?)),
					val: None,
				};
			} else if self.consume(">") {
				node = Node {
					kind: NodeKind::Gt,
					lhs: Some(Box::new(node)),
					rhs: Some(Box::new(self.add()?)),
					val: None,
				};
			} else if self.consume(">=") {
				node = Node {
					kind: NodeKind::Ge,
					lhs: Some(Box::new(node)),
					rhs: Some(Box::new(self.add()?)),
					val: None,
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
				};
				} else if self.consume("-") {
				node = Node {
					kind: NodeKind::Sub,
					lhs: Some(Box::new(node)),
					rhs: Some(Box::new(self.mul()?)),
					val: None,
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
				};
			} else if self.consume("/") {
				node = Node {
					kind: NodeKind::Div,
					lhs: Some(Box::new(node)),
					rhs: Some(Box::new(self.unary()?)),
					val: None,
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
				})),
				rhs: Some(Box::new(self.primary()?)),
				val: None,
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
		} else {
			Ok(Node {
				kind: NodeKind::Num,
				lhs: None,
				rhs: None,
				val: Some(self.expect_number()?),
			})
		}
	}
}
