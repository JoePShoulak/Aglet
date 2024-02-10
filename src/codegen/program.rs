use crate::parser::ast::Program;

impl Program {
	pub fn codegen(&self) {
		for stmt in &self.stmts {
			stmt.codegen();
		}
	}
}
