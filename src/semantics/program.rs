use crate::parser::ast::Program;
use crate::semantics::Analyzer;

impl Program {
	pub fn analyze(&self, analyzer: &mut Analyzer) {
		for stmt in &self.stmts {
			stmt.analyze(analyzer);
		}
	}
}
