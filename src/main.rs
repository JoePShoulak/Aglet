use std::io::Read;
use std::process::ExitCode;

mod lexer;
mod parser;
pub mod message;

fn main() -> ExitCode {
	//Read program text from stdin.
	//Consider handling possible error so pgm doesn't panic?
	let mut s = String::new();
	std::io::stdin().read_to_string(&mut s).unwrap();

	//Create lexer (iterator), with debug info for each token read
	let lexer = lexer::Lexer::new(&s);//.inspect(|tok| eprintln!("tok: {:?}", tok));

	message::info("Building AST...");

	//Read input, splitting into tokens as it's read.
	let _ast = match parser::parse(lexer) {
		Err(e) => {
			match e.0 {
				None => {
					//We hit EOF
					message::eof_error(None, &s, format!("{}", e.1));
				},
				Some(s) => {
					message::error(format!("{}", e.1), s.1);
				},
			};

			None
		},
		Ok(program) => Some(program),
	};

	if message::errored() {
		message::print_all(s, None);
		return ExitCode::FAILURE;
	}

	message::info("Finished compilation.");
	return ExitCode::SUCCESS;
}
