use std::io::Read;
use std::process::ExitCode;
use std::fs;
use std::env;

mod lexer;
mod parser;
pub mod message;

fn main() -> ExitCode {
	let mut s = String::new();
	let mut filename: Option<String> = None;

	//If an argument is given, assume that's the input file.
	//Otherwise, read program text from stdin.
	let args: Vec<String> = env::args().collect();

	match args.get(1) {
		None => {
			std::io::stdin().read_to_string(&mut s).unwrap();
		},
		Some(fname) => {
			s = match fs::read_to_string(fname) {
				Ok(file_contents) => file_contents,
				Err(error) => {
					eprintln!("Error reading file: {}", error);
					return ExitCode::FAILURE;
				}
			};
			filename = Some(fname.to_string());
		},
	}

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
		message::print_all(s, filename);
		return ExitCode::FAILURE;
	}

	message::info("Finished compilation.");
	return ExitCode::SUCCESS;
}
