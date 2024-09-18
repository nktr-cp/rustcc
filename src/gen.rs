use crate::error;
use crate::parser::{BinaryOpKind, ComparisonOpKind, Node, NodeKind, UnaryOpKind};

fn gen_lval(node: &Node) {
    match &node.kind {
        NodeKind::LVar(lvar) => {
            println!("  mov rax, rbp");
            println!("  sub rax, {}", lvar.offset);
            println!("  push rax");
        }
        _ => {
            error::error("代入の左辺値が変数ではありません");
        }
    }
}

pub fn gen(node: &Node, id: &mut i32) {
    match &node.kind {
        NodeKind::Num(val) => {
            println!("  push {}", val);
            return;
        }
        NodeKind::LVar(_lvar) => {
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
            gen(
                node.rhs
                    .as_ref()
                    .unwrap()
                    .rhs
                    .as_ref()
                    .unwrap()
                    .rhs
                    .as_ref()
                    .unwrap(),
                id,
            ); // body
            gen(
                node.rhs
                    .as_ref()
                    .unwrap()
                    .rhs
                    .as_ref()
                    .unwrap()
                    .lhs
                    .as_ref()
                    .unwrap(),
                id,
            ); // inc
            println!("  jmp .Lbegin{}", local_id);
            println!(".Lend{}:", local_id);
        }
        NodeKind::Block(stmts) => {
            for (i, stmt) in stmts.iter().enumerate() {
                gen(stmt, id);
                if i != stmts.len() - 1 {
                    println!("  pop rax");
                }
            }
        }
        NodeKind::Fncall(lvar, args) => {
            const REGS: [&str; 6] = ["rdi", "rsi", "rdx", "rcx", "r8", "r9"];

            for (i, arg) in args.iter().enumerate() {
                gen(arg, id);
                println!("  pop {} # set {}-th argument", REGS[i], i);
            }
            println!("  mov rax, {}", args.len());

            // rspの位置を調整
            // r10に調整分を保存
            println!("  mov r10, rsp");
            println!("  and r10, 15 # save offset to r10");
            println!("  sub rsp, r10 # align rsp to be divisible by 16");
            println!("  call {}", lvar.name);
            println!("  add rsp, r10 # adjust stack pointer after call");

            println!("  push rax # rax has return value after call");
        }
        NodeKind::Fndef(name, args, locals) => {
            const REGS: [&str; 6] = ["rdi", "rsi", "rdx", "rcx", "r8", "r9"];

            println!(".global {}", name);
            println!("{}:", name);

            // prologue
            println!("  push rbp");
            println!("  mov rbp, rsp # save base pointer");
            println!(
                "  sub rsp, {} # make spaces for local variables",
                locals.len() * 8 + args.len() * 8 + 8
            );

            // save arguments to local variables
            for (i, _arg) in args.iter().enumerate() {
                let offset = 8 * i + 8;
                println!("  mov rax, rbp");
                println!("  mov [rax-{}], {} # push argument", offset, REGS[i]);
            }

            gen(node.rhs.as_ref().unwrap(), id);
            println!("  pop rax");

            // epilogue
            println!("  mov rsp, rbp # restore stack pointer");
            println!("  pop rbp # discard base pointer");
            println!("  ret");
            return;
        }
        NodeKind::UnaryOp(op) => match op {
            UnaryOpKind::Ref => {
                gen_lval(node.lhs.as_ref().unwrap());
                return;
            }
            UnaryOpKind::Deref => {
                gen(node.lhs.as_ref().unwrap(), id);
                println!("  pop rax");
                println!("  mov rax, [rax]");
                println!("  push rax");
                return;
            }
        },
        _ => {
            gen(node.lhs.as_ref().unwrap(), id);
            gen(node.rhs.as_ref().unwrap(), id);

            println!("  pop rdi");
            println!("  pop rax");

            match &node.kind {
                NodeKind::BinaryOp(op) => match op {
                    BinaryOpKind::Add => println!("  add rax, rdi"),
                    BinaryOpKind::Sub => println!("  sub rax, rdi"),
                    BinaryOpKind::Mul => println!("  imul rax, rdi"),
                    BinaryOpKind::Div => {
                        println!("  cqo");
                        println!("  idiv rdi");
                    }
                },
                NodeKind::Comparison(op) => match op {
                    ComparisonOpKind::Eq => {
                        println!("  cmp rax, rdi");
                        println!("  sete al");
                        println!("  movzb rax, al");
                    }
                    ComparisonOpKind::Nq => {
                        println!("  cmp rax, rdi");
                        println!("  setne al");
                        println!("  movzb rax, al");
                    }
                    ComparisonOpKind::Lt => {
                        println!("  cmp rax, rdi");
                        println!("  setl al");
                        println!("  movzb rax, al");
                    }
                    ComparisonOpKind::Le => {
                        println!("  cmp rax, rdi");
                        println!("  setle al");
                        println!("  movzb rax, al");
                    }
                    ComparisonOpKind::Gt => {
                        println!("  cmp rax, rdi");
                        println!("  setg al");
                        println!("  movzb rax, al");
                    }
                    ComparisonOpKind::Ge => {
                        println!("  cmp rax, rdi");
                        println!("  setge al");
                        println!("  movzb rax, al");
                    }
                },
                _ => unreachable!(),
            }

            println!("  push rax");
        }
    }
}
