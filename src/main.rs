use std::process::ExitCode;
use std::fs;

mod lexer;
mod parser;
pub mod message;
mod flags;

fn main() -> ExitCode {
	let options = flags::read();

	//Read input file
	let mut s = match fs::read_to_string(&options.input) {
		Ok(file_contents) => file_contents,
		Err(error) => {
			eprintln!("Error reading file {:?}: {}", options.input, error);
			return ExitCode::FAILURE;
		}
	};

	s = s.replace("\t", " "); //For formatting reasons, replace all tabs with spaces.

	//Create lexer (iterator), with debug info for each token read
	let lexer = lexer::Lexer::new(&s);//.inspect(|tok| eprintln!("tok: {:?}", tok));

	message::info("Building AST...");

	//Read input, splitting into tokens as it's read.
	let ast = match parser::parse(lexer) {
		Err(e) => {
			match e.0 {
				None => {
					//We hit EOF
					message::eof_error(&Some(options.input.to_str().unwrap().to_string()), &s, format!("{}", e.1));
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
		message::abort();
		message::print_all(s, &Some(options.input.to_str().unwrap().to_string()));
		return ExitCode::FAILURE;
	}

	if options.ast {
		println!("{}", parser::pretty(ast));
	}

	message::info("Finished compilation.");
	return ExitCode::SUCCESS;
}
