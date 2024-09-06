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
				NodeKind::Eq => {
					println!("  cmp rax, rdi");
					println!("  sete al");
					println!("  movzb rax, al");
				}
				NodeKind::Nq => {
					println!("  cmp rax, rdi");
					println!("  setne al");
					println!("  movzb rax, al");
				}
				NodeKind::Lt => {
					println!("  cmp rax, rdi");
					println!("  setl al");
					println!("  movzb rax, al");
				}
				NodeKind::Le => {
					println!("  cmp rax, rdi");
					println!("  setle al");
					println!("  movzb rax, al");
				}
				NodeKind::Gt => {
					println!("  cmp rax, rdi");
					println!("  setg al");
					println!("  movzb rax, al");
				}
				NodeKind::Ge => {
					println!("  cmp rax, rdi");
					println!("  setge al");
					println!("  movzb rax, al");
				}
				_ => unreachable!(),
			}

			println!("  push rax");
		}
	}

}
