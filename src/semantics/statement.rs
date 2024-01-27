use crate::parser::ast::Statement;
use crate::parser::ast::Stmt::*;
use crate::message::Context;
use crate::message;
use crate::types;

impl Statement {
	pub fn analyze(&self, context: &Context) {
		match &self.node {
			ExprStmt(expr) => {
				expr.analyze(context);
			},

			//In release builds, not having semantic analysis for any node type should be an error
			#[cfg(debug_assertions)]
			a => {
				message::warning(format!("No semantic analysis defined for variant of node `{}`", types::basename(a)), Some(self.span), Some(context));
			},
		}
	}
}
