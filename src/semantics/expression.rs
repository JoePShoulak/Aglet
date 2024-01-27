use crate::parser::ast::Expression;
use crate::parser::ast::Expr::*;
use crate::semantics::Analyzer;
use crate::message;
use crate::types;

impl Expression {
	pub fn analyze(&self, analyzer: &mut Analyzer) {
		match &self.node {
			Integer(value) => {
				//Warn if the value will not fit in 2 bytes
				if *value >= 32768 {
					message::error("Integer value exceeds the size of its type!".to_string(), Some(self.span), Some(analyzer.context));
				}
			},

			Add(a, b) => {
				a.analyze(analyzer);
				b.analyze(analyzer);
			},

			#[cfg(debug_assertions)]
			a => {
				message::warning(format!("No semantic analysis defined for variant of node `{}`", types::basename(a)), Some(self.span), Some(analyzer.context));
			}
		}
	}
}
