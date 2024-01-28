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

			FuncDecl(name, params, return_type, body) => {
				match analyzer.get_function(&name.value) {
					Some(_) => {
						message::error(format!("Redeclaration of function `{}`", name.value), Some(name.span), Some(analyzer.context));
					},
					None => {
						let params = params.iter().map(|s| s.datatype.clone()).collect();
						analyzer.set_function(&name.value, params, &return_type.value);
					},
				}

				if !analyzer.valid_return_type(&return_type.value) {
					message::error(format!("Unknown return type `{}`. Valid types are `int` or `void`", return_type.value), Some(return_type.span), Some(analyzer.context));
				}

				if name.value == "main" {
					//Force the main() function to have a specific signature
					if params.len() > 0 || return_type.value != "void" {
						message::error(format!("Function signature for `main` must be `() -> void`"), Some(name.span), Some(analyzer.context))
					}

					//Also only allow it to be declared in the global scope
					if analyzer.scopes.len() > 1 {
						message::error(format!("Function `main` may only be declared in the global scope"), Some(name.span), Some(analyzer.context));
					}
				}

				if body.stmts.len() > 0 {
					analyzer.push_scope();
					body.analyze(analyzer);
					analyzer.pop_scope();
				}
			},

			//In release builds, not having semantic analysis for any node type should be an error
			#[cfg(debug_assertions)]
			a => {
				message::warning(format!("No semantic analysis defined for variant of node `{}`", types::basename(a)), Some(self.span), Some(analyzer.context));
			},
		}
	}
}