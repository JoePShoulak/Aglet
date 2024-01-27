use crate::parser::ast::Statement;
use crate::parser::ast::Stmt::*;
use crate::semantics::Analyzer;
use crate::message;
use crate::types;

impl Statement {
	pub fn analyze(&self, analyzer: &mut Analyzer) {
		match &self.node {
			ExprStmt(expr) => {
				expr.analyze(analyzer);
			},

			//In release builds, not having semantic analysis for any node type should be an error
			#[cfg(debug_assertions)]
			a => {
				message::warning(format!("No semantic analysis defined for variant of node `{}`", types::basename(a)), Some(self.span), Some(analyzer.context));
			},
		}
	}
}
