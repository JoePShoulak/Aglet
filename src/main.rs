use std::process::ExitCode;
use std::fs;

mod lexer;
mod parser;
mod semantics;

pub mod message;
mod flags;
mod types;

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

	let filename = options.input.to_str().unwrap().to_string();
	let context = message::Context { filename: &filename, source: &s };

	//Create lexer (iterator), with debug info for each token read
	let lexer = lexer::Lexer::new(&context);//.inspect(|tok| eprintln!("tok: {:?}", tok));

	message::info("Building AST...");

	//Read input, splitting into tokens as it's read.
	let ast = match parser::parse(lexer) {
		Err(e) => {
			match e.0 {
				None => {
					//We hit EOF
					message::error(format!("{}", "Unexpected end of file"), None, Some(&context));
				},
				Some(s) => {
					message::error(format!("{}", e.1), Some(s.1), Some(&context));
				},
			};

			None
		},
		Ok(program) => Some(program),
	};

	if message::errored() {
		message::abort();
		return ExitCode::FAILURE;
	}

	let ast = ast.unwrap();

	//--ast flag is only available in debug builds
	#[cfg(debug_assertions)]
	if options.ast {
		println!("{}", parser::pretty(&ast));
	}

	message::info("Running semantic analysis...");
	let _analysis = semantics::Analyzer::run(&ast, &context);

	if message::errored() {
		message::abort();
		return ExitCode::FAILURE;
	}

	message::info("Finished compilation.");
	return ExitCode::SUCCESS;
}
