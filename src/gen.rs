use crate::error;
use crate::parser::{Node, NodeKind};

fn gen_lval(node: &Node) {
    if node.kind != NodeKind::Lvar {
        error::error("代入の左辺値が変数ではありません");
    }

    println!("  mov rax, rbp");
    println!("  sub rax, {}", node.lvar.clone().unwrap().offset);
    println!("  push rax");
}

pub fn gen(node: &Node, id: &mut i32) {
	match node.kind {
			NodeKind::Num => {
				println!("  push {}", node.val.unwrap());
				return;
			}
			NodeKind::Lvar => {
				gen_lval(node);
				println!("  pop rax");
				println!("  mov rax, [rax]");
				println!("  push rax");
				return;
			}
			NodeKind::Assign => {
				gen_lval(node.lhs.as_ref().unwrap());
				gen(node.rhs.as_ref().unwrap(), id);

				println!("  pop rdi");
				println!("  pop rax");
				println!("  mov [rax], rdi");
				println!("  push rdi");
				return;
			}
			NodeKind::Return => {
				gen(node.lhs.as_ref().unwrap(), id);
				println!("  pop rax");
				println!("  mov rsp, rbp");
				println!("  pop rbp");
				println!("  ret");
				return;
			}
			NodeKind::If => {
				let local_id = *id;
				*id += 1;
				gen(node.lhs.as_ref().unwrap(), id); // cond
				println!("  pop rax");
				println!("  cmp rax, 0");
				if node.rhs.as_ref().unwrap().kind == NodeKind::Else {
					println!("  je .Lelse{}", local_id);
					gen(node.rhs.as_ref().unwrap().lhs.as_ref().unwrap(), id); // then
					println!("  jmp .Lend{}", local_id);
					println!(".Lelse{}:", local_id);
					gen(node.rhs.as_ref().unwrap().rhs.as_ref().unwrap(), id); // else
					println!(".Lend{}:", local_id);
				} else {
					println!("  je .Lend{}", local_id);
					gen(node.rhs.as_ref().unwrap(), id); // then
					println!(".Lend{}:", local_id);
				}
				return;
			}
			NodeKind::While => {
				let local_id = *id;
				*id += 1;
				println!(".Lbegin{}:", local_id);
				gen(node.lhs.as_ref().unwrap(), id); // cond
				println!("  pop rax");
				println!("  cmp rax, 0");
				println!("  je .Lend{}", local_id);
				gen(node.rhs.as_ref().unwrap(), id); // body
				println!("  jmp .Lbegin{}", local_id);
				println!(".Lend{}:", local_id);
			}
			NodeKind::For => {
				let local_id = *id;
				*id += 1;
				gen(node.lhs.as_ref().unwrap(), id); // init
				println!(".Lbegin{}:", local_id);
				gen(node.rhs.as_ref().unwrap().lhs.as_ref().unwrap(), id); // cond
				println!("  pop rax");
				println!("  cmp rax, 0");
				println!("  je .Lend{}", local_id);	
				gen(node.rhs.as_ref().unwrap().rhs.as_ref().unwrap().rhs.as_ref().unwrap(), id); // body
				gen(node.rhs.as_ref().unwrap().rhs.as_ref().unwrap().lhs.as_ref().unwrap(), id); // inc
				println!("  jmp .Lbegin{}", local_id);
				println!(".Lend{}:", local_id);
			}
			NodeKind::Block => {
				let stmts = node.stmts.as_ref().unwrap();
				for (i, stmt) in stmts.iter().enumerate() {
					gen(stmt, id);
					if i != stmts.len() - 1 {
						println!("  pop rax");
					}
				}
			}
			NodeKind::Fncall => {
				const REGS: [&str; 6] = ["rdi", "rsi", "rdx", "rcx", "r8", "r9"];

				let args = node.stmts.as_ref().unwrap();
				for (i, arg) in args.iter().enumerate() {
					gen(arg, id);
					println!("  pop {}", REGS[i]);
				}
				println!("  mov rax, {}", args.len());

				// rspの位置を調整
				println!("  and rsp, -16");
				println!("  call {}", node.lvar.clone().unwrap().name);

				println!("  push rax");
			}
			_ => {
				gen(node.lhs.as_ref().unwrap(), id);
				gen(node.rhs.as_ref().unwrap(), id);

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
