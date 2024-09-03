use std::env;
use std::process;

fn main() {
	let args: Vec<String> = env::args().collect();

	if args.len() != 2 {
		eprintln!("引数の個数が正しくありません");
		process::exit(1);
	}

	let number: i32 = match args[1].parse() {
		Ok(num) => num,
		Err(_) => {
			eprintln!("引数には数値を指定してください");
			process::exit(1);
		}
	};

	println!(".intel_syntax noprefix");
	println!(".globl main");
	println!("main:");
	println!("	mov rax, {}", number);
	println!("	ret");
}
