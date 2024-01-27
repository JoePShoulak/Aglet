use crate::parser::ast::Program;
use crate::message::Context;

impl Program {
	pub fn analyze(&self, context: &Context) {
		for stmt in &self.stmts {
			stmt.analyze(context);
		}
	}
}
