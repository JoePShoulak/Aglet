use crate::parser::ast::Program;
use crate::semantics::Analyzer;

impl Program {
	pub fn analyze(&self, analyzer: &mut Analyzer) -> bool {
		//return true if this program is guaranteed to return
		self.stmts.iter().map(|s| s.analyze(analyzer)).fold(false, |a, b| a || b)
	}
}
