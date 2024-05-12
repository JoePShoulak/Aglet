pub enum Bytecode {
	LDA(String),
	STA(String),
	JSR(String),
}

use Bytecode::*;

impl Bytecode {
	pub fn text(self) -> String {
		match self {
			LDA(address) => {
				format!("lda {}", address)
			},

			STA(address) => {
				format!("sta {}", address)
			},

			JSR(label) => {
				format!("sta {}", label)
			}
		}
	}

	pub fn binary(self) -> String {
		panic!("Binary bytecode output is not implemented!")
	}

	pub fn output_text(bytecode: Vec<Bytecode>) -> String {
		let result: Vec<String> = bytecode.into_iter().map(|bc| bc.text()).collect();
		return result.join("");
	}

	pub fn output_binary(bytecode: Vec<Bytecode>) -> String {
		let result: Vec<String> = bytecode.into_iter().map(|bc| bc.binary()).collect();
		return result.join("");
	}
}
