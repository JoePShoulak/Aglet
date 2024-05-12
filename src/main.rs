use std::fs;
use std::io;
use std::process::ExitCode;

mod codegen;
mod codegen;
mod lexer;
mod parser;
mod semantics;
use codegen::asm::Bytecode;

mod flags;
pub mod message;

fn main() -> ExitCode {
	//Disable colors globally if stderr or stdout are not TTY
	if !atty::is(atty::Stream::Stdout) || !atty::is(atty::Stream::Stderr) {
		colored::control::set_override(false);
	}

	let options = flags::read();

	if options.language_server {
		*message::LANGUAGE_SERVER.lock().unwrap() = true;
	}

	//Read input file
	let mut s = String::new();
	let filename = if options.input.to_str().unwrap() == "-" {
		for line in io::stdin().lines() {
			s += &line.unwrap();
		}
		"stdin"
	} else {
		s = match fs::read_to_string(&options.input) {
			Ok(file_contents) => file_contents,
			Err(error) => {
				eprintln!("Error reading file {:?}: {}", options.input, error);
				return ExitCode::FAILURE;
			}
		};
		options.input.to_str().unwrap()
	}
	.to_string();

	s = s.replace("\t", " "); //For formatting reasons, replace all tabs with spaces.

	let context = message::Context {
		filename: &filename,
		source: &s,
	};

	//Create lexer (iterator), with debug info for each token read
	let lexer = lexer::Lexer::new(&context); //.inspect(|tok| eprintln!("tok: {:?}", tok));

	message::info("Building AST...");

	//Read input, splitting into tokens as it's read.
	let ast = match parser::parse(lexer) {
		Err(e) => {
			match e.0 {
				None => {
					//We hit EOF
					message::error(
						format!("{}", "Unexpected end of file"),
						None,
						Some(&context),
					);
				}
				Some(s) => {
					message::error(format!("{}", e.1), Some(s.1), Some(&context));
				}
			};

			None
		}
		Ok(program) => Some(program),
	};

	if message::errored() {
		message::abort();
		return if options.language_server {
			ExitCode::SUCCESS
		} else {
			ExitCode::FAILURE
		};
	}

	let ast = ast.unwrap();

	//--ast flag is only available in debug builds
	#[cfg(debug_assertions)]
	if options.ast {
		println!("{}", parser::pretty(&ast));
	}

	message::info("Running semantic analysis...");
	let _analysis = semantics::Analyzer::run(&ast, &context, &options);

	if message::errored() {
		message::abort();
		return if options.language_server {
			ExitCode::SUCCESS
		} else {
			ExitCode::FAILURE
		};
	}

	//If we're running as a language server, exit here.
	//No need to generate code.
	if options.language_server {
		return ExitCode::SUCCESS;
	}

	//Program is OK, generate code.
	let bytecode = ast.codegen();

	//Here is where we'd run bytecode optimizations if we had them.

	//Convert code to text
	let output_text = if options.binary {
		Bytecode::output_binary(bytecode)
	} else {
		Bytecode::output_text(bytecode)
	};

	//Output to file or stdout
	let filename = options.output.to_str().unwrap();
	if filename == "-" {
		print!("{}", output_text);
	} else {
		match fs::write(filename, output_text) {
			Err(error) => {
				eprintln!("{}", error);
				return ExitCode::FAILURE;
			}

			Ok(_) => {}
		}
	}

	message::info("Finished compilation.");
	return ExitCode::SUCCESS;
}
