use crate::error;
use crate::parser::get_type_size;
use crate::parser::{BinaryOpKind, ComparisonOpKind, Node, NodeKind, TypeKind, UnaryOpKind};

fn gen_lval(node: &Node, id: &mut i32) {
    match &node.kind {
        NodeKind::LVar(lvar) => {
            println!("  mov rax, rbp");
            println!("  sub rax, {}", lvar.offset);
            println!("  push rax");
        }
        // デリファレンスの場合は右辺値を生成
        // genを呼んでアドレスをraxに詰める
        NodeKind::UnaryOp(op) => {
            if *op == UnaryOpKind::Deref {
                gen(node.lhs.as_ref().unwrap(), id);
            }
        }
        NodeKind::GVar(gvar) => {
            println!("  lea rax, {}[rip]", gvar.name);
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
        NodeKind::LVarDef(_) => {}
        NodeKind::LVar(_lvar) => {
            gen_lval(node, id);
            println!("  pop rax");
            if node.ty.kind == TypeKind::Char {
                println!("  movzx rax, BYTE PTR [rax]");
            } else {
                println!("  mov rax, [rax]");
            }
            println!("  push rax");
            return;
        }
        NodeKind::GVarDef(gvar) => {
            println!("  .bss");
            println!("  .global {}", gvar.name);
            println!("{}:", gvar.name);
            println!("  .zero {}\n", get_type_size(&gvar.ty)); // 初期化はサポートしてないので0埋め
            return;
        }
        NodeKind::GVar(gvar) => {
            gen_lval(node, id);
            //　配列の場合は中身を参照しない
            // なんでGVarのときだけこの処理が必要なのかはよくわかってない (アドレッシングモードの違い？)
            if gvar.ty.kind == TypeKind::Arr {
                return;
            }
            println!("  pop rax");
            if node.ty.kind == TypeKind::Char {
                println!("  movzx rax, BYTE PTR [rax]");
            } else {
                println!("  mov rax, [rax]");
            }
            println!("  push rax");
            return;
        }
        NodeKind::Assign => {
            gen_lval(node.lhs.as_ref().unwrap(), id);
            gen(node.rhs.as_ref().unwrap(), id);

            println!("  pop rdi");
            println!("  pop rax");
            if node.lhs.as_ref().unwrap().ty.kind == TypeKind::Char {
                println!("  mov BYTE PTR [rax], dil");
            } else {
                println!("  mov [rax], rdi");
            }
            println!("  push rdi\n");
            return;
        }
        NodeKind::Return => {
            gen(node.lhs.as_ref().unwrap(), id);
            println!("  pop rax");
            println!("  mov rsp, rbp");
            println!("  pop rbp");
            println!("  ret\n");
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
            for stmt in stmts.iter() {
                gen(stmt, id);
            }
        }
        NodeKind::Fncall(func, args) => {
            const REGS: [&str; 6] = ["rdi", "rsi", "rdx", "rcx", "r8", "r9"];

            for (i, arg) in args.iter().enumerate() {
                gen(arg, id);
                println!("  pop {} # set {}-th argument", REGS[i], i);
            }
            println!("  mov rax, {}", args.len());

            println!("  mov al, 0");

            // rspの位置を調整
            // r10に調整分を保存
            println!("  mov r10, rsp");
            println!("  sub r10, 8");
            println!("  and r10, 15 # save offset to r10");
            println!("  sub rsp, r10 # align rsp to be divisible by 16");
            println!("  push r10 # save offset to stack");
            println!("  call {}", func.name);
            println!("  pop r10 # restore offset from stack");
            println!("  add rsp, r10 # adjust stack pointer after call");

            println!("  push rax # rax has return value after call");
        }
        NodeKind::Fndef(func, args) => {
            const REGS: [&str; 6] = ["rdi", "rsi", "rdx", "rcx", "r8", "r9"];

            println!("  .text");
            println!("  .global {}", func.name);
            println!("{}:", func.name);

            // prologue
            println!("  push rbp");
            println!("  mov rbp, rsp # save base pointer");
            println!(
                "  sub rsp, {} # make spaces for local variables\n",
                func.stack_size
            );

            // save arguments to local variables
            for (i, arg) in args.iter().enumerate() {
                let mut offset = 0;
                match &arg.kind {
                    NodeKind::LVar(lvar) => {
                        offset = lvar.offset;
                    }
                    _ => {
                        error::error("関数の引数が変数ではありません");
                    }
                }
                println!("  mov [rbp-{}], {} # push argument", offset, REGS[i]);
            }

            gen(node.rhs.as_ref().unwrap(), id);
            println!("  pop rax");

            // epilogue
            println!("\n  mov rsp, rbp # restore stack pointer");
            println!("  pop rbp # discard base pointer");
            println!("  ret");
            return;
        }
        NodeKind::UnaryOp(op) => match op {
            UnaryOpKind::Ref => {
                gen_lval(node.lhs.as_ref().unwrap(), id);
                return;
            }
            UnaryOpKind::Deref => {
                gen(node.lhs.as_ref().unwrap(), id);
                if node.ty.kind == TypeKind::Arr {
                    return;
                }
                println!("  pop rax");
                if node.ty.kind == TypeKind::Char {
                    println!("  movzx rax, BYTE PTR [rax]");
                } else {
                    println!("  mov rax, [rax]");
                }
                println!("  push rax");
                return;
            }
        },
        NodeKind::Strlit(lit) => {
            println!("  lea rax, .LC{}[rip]", lit.idx);
            println!("  push rax");
            return;
        }
        _ => {
            gen(node.lhs.as_ref().unwrap(), id);
            gen(node.rhs.as_ref().unwrap(), id);

            println!("  pop rdi");
            println!("  pop rax");

            match &node.kind {
                NodeKind::BinaryOp(op) => match op {
                    BinaryOpKind::Add => {
                        if matches!(node.ty.kind, TypeKind::Ptr | TypeKind::Arr) {
                            if matches!(
                                node.lhs.as_ref().unwrap().ty.kind,
                                TypeKind::Ptr | TypeKind::Arr
                            ) {
                                println!(
                                    "  imul rdi, {}",
                                    get_type_size(&node.ty.ptr_to.as_ref().unwrap())
                                );
                            } else {
                                println!(
                                    "  imul rax, {}",
                                    get_type_size(&node.ty.ptr_to.as_ref().unwrap())
                                );
                            }
                        }

                        println!("  add rax, rdi");
                    }
                    BinaryOpKind::Sub => {
                        if matches!(node.ty.kind, TypeKind::Ptr | TypeKind::Arr) {
                            if matches!(
                                node.lhs.as_ref().unwrap().ty.kind,
                                TypeKind::Ptr | TypeKind::Arr
                            ) {
                                println!(
                                    "  imul rdi, {}",
                                    get_type_size(&node.ty.ptr_to.as_ref().unwrap())
                                );
                            } else {
                                println!(
                                    "  imul rax, {}",
                                    get_type_size(&node.ty.ptr_to.as_ref().unwrap())
                                );
                            }
                        }

                        println!("  sub rax, rdi");
                    }
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
