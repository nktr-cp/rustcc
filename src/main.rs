use std::env;
use std::process;

fn main() {
	let args: Vec<String> = env::args().collect();

	if args.len() != 2 {
		eprintln!("引数の個数が正しくありません");
		process::exit(1);
	}

	let input = &args[1];

	println!(".intel_syntax noprefix");
	println!(".globl main");
	println!("main:");

	let mut chars = input.chars().peekable();
	let mut current_num = String::new();

	while let Some(&c) = chars.peek() {
		if c.is_digit(10) {
			current_num.push(chars.next().unwrap());
		} else {
			break;
		}
	}

	if !current_num.is_empty() {
		println!("  mov rax, {}", current_num);
		current_num.clear();
	} else {
		eprintln!("式は数字で始まる必要があります");
		process::exit(1);
	}

	while let Some(c) = chars.next() {
		match c {
			'+' | '-' => {
				let op = c;
				while let Some(&next) = chars.peek() {
					if next.is_digit(10) {
						current_num.push(chars.next().unwrap());
					} else {
						break;
					}
				}

				if !current_num.is_empty() {
					match op {
						'+' => println!("  add rax, {}", current_num),
						'-' => println!("  sub rax, {}", current_num),
						_ => unreachable!(),
					}
					current_num.clear();
				} else {
					eprintln!("演算子の後には数字が必要です");
					process::exit(1);
				}
			},
			_ => {
				eprintln!("予期しない文字です: '{}'", c);
				process::exit(1);
			}
		}
	}

	println!("  ret");
}
