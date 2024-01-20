use std::io::Read;
use std::process::ExitCode;

mod lexer;
pub mod parser;

fn main() -> ExitCode {
	//Read program text from stdin.
	//Consider handling possible error so pgm doesn't panic?
	let mut s = String::new();
	std::io::stdin().read_to_string(&mut s).unwrap();

	//Create lexer (iterator), with debug info for each token read
	let lexer = lexer::Lexer::new(&s).inspect(|tok| eprintln!("tok: {:?}", tok));

	//Read input, splitting into tokens as it's read.
	//Note: This can panic! Consider handling it so program can exit gracefully.
	let _ast = parser::parse(lexer).unwrap();

	return ExitCode::SUCCESS;
}
