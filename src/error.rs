pub fn error(message: &str) {
		eprintln!("{}", message);
		std::process::exit(1);
}