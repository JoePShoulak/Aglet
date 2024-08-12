use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
	name = "Aglet Compiler",
	about = "A modern language for an old system;\nCompiles Aglet source to 6502 assembly."
)]
pub struct Options {
	/// Prints the abstract syntax tree
	#[cfg(debug_assertions)]
	#[structopt(long)]
	pub ast: bool,

	/// Suppress warnings
	#[structopt(long, short)]
	pub warn_suppress: bool,

	/// Output raw binary instead of assembly
	#[structopt(long, short)]
	pub binary: bool,

	/// Output detailed info in an easy-to-parse format
	#[structopt(long)]
	pub language_server: bool,

	/// The input file
	#[structopt(parse(from_os_str))]
	pub input: PathBuf,

	/// The output file
	#[structopt(parse(from_os_str))]
	pub output: PathBuf,
}

pub fn read() -> Options {
	return Options::from_args();
}
