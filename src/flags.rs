use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "Aglet Compiler", about = "A modern language for an old system;\nCompiles Aglet source to 6502 assembly.")]
pub struct Options {
	/// Prints the abstract syntax tree
	#[cfg(debug_assertions)]
	#[structopt(long)]
	pub ast: bool,

	/// Suppress warnings
	#[cfg(debug_assertions)]
	#[structopt(long, short)]
	pub warn_suppress: bool,

	/// The input file
	#[structopt(parse(from_os_str))]
	pub input: PathBuf,
}

pub fn read() -> Options {
	return Options::from_args();
}
