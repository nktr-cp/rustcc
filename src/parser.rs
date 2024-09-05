use crate::tokenizer::{Token, TokenKind};

#[derive(Debug, Clone)]
pub enum NodeKind {
	Add,
	Sub,
	Mul,
	Div,
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

	fn consume(&mut self, op: char) -> bool {
		if self.tokens[self.pos].kind != TokenKind::Reserved || self.tokens[self.pos].str != op.to_string() {
			return false;
		}
		self.pos += 1;
		true
	}

	fn expect(&mut self, op: char) -> Result<(), String> {
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
		let mut node = self.mul()?;

		loop {
			if self.consume('+') {
				node = Node {
					kind: NodeKind::Add,
					lhs: Some(Box::new(node)),
					rhs: Some(Box::new(self.mul()?)),
					val: None,
				};
				} else if self.consume('-') {
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
		let mut node = self.primary()?;

		loop {
			if self.consume('*') {
				node = Node {
					kind: NodeKind::Mul,
					lhs: Some(Box::new(node)),
					rhs: Some(Box::new(self.primary()?)),
					val: None,
				};
			} else if self.consume('/') {
				node = Node {
					kind: NodeKind::Div,
					lhs: Some(Box::new(node)),
					rhs: Some(Box::new(self.primary()?)),
					val: None,
				};
			} else {
				return Ok(node);
			}
		}	
	}

	fn primary(&mut self) -> Result<Node, String> {
		if self.consume('(') {
			let node = self.expr()?;
			self.expect(')')?;
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
