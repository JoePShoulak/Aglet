use crate::parser::ast::Program;
use super::asm::Bytecode;

impl Program {
	pub fn codegen(&self) -> Vec<Bytecode> {
		self.stmts.iter().flat_map(|stmt| stmt.codegen()).collect()
	}
}
