use crate::parser::{Node, NodeKind};

pub fn gen(node: &Node) {
	match node.kind {
		NodeKind::Num => {
			println!("  push {}", node.val.unwrap());
			return;
		}
		_ => {
			gen(node.lhs.as_ref().unwrap());
			gen(node.rhs.as_ref().unwrap());

			println!("  pop rdi");
			println!("  pop rax");

			match node.kind {
				NodeKind::Add => println!("  add rax, rdi"),
				NodeKind::Sub => println!("  sub rax, rdi"),
				NodeKind::Mul => println!("  imul rax, rdi"),
				NodeKind::Div => {
					println!("  cqo");
					println!("  idiv rdi");
				}
				_ => unreachable!(),
			}

			println!("  push rax");
		}
	}

}
