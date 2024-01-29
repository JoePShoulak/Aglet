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
				if *value > 32767 {
					message::error("Value exceeds the maximum for a signed 2-byte integer (max 32767)".to_string(), Some(self.span), Some(analyzer.context));
				}
			},

			Neg(expr) => {
				match &expr.node {
					//Negative integers have a different "max" than positive, by 1.
					Integer(value) => {
						if *value > 32768 {
							message::error("Value exceeds the minimum for a signed 2-byte integer (min -32768)".to_string(), Some(self.span), Some(analyzer.context));
						}
					},
					_ => {
						expr.analyze(analyzer);
					},
				}
			}

			Add(a, b) => {
				a.analyze(analyzer);
				b.analyze(analyzer);
			},

			FuncCall(name, params) => {
				match &name.node {
					Var(id) => {
						for param in params.iter() {
							param.analyze(analyzer);
						}

						match analyzer.get_function(id) {
							None => {
								message::error(format!("Use of undeclared function `{}`", id), Some(name.span), Some(analyzer.context));
							},

							Some(func) => {
								let ct = func.param_types.len();
								if params.len() != ct {
									message::error(format!("Expected {} argument{} to function `{}`, got {}", ct, if ct == 1 {""} else {"s"}, id, params.len()), Some(self.span), Some(analyzer.context));
									message::hint(format!("Function signature is `{}{}`", id, func), Some(self.span), Some(analyzer.context));
								}
							}
						}
					},

					_ => {
						message::error("Composite function names are not supported yet".to_string(), Some(name.span), Some(analyzer.context));
					}
				}
			},

			Var(name) => {
				match analyzer.get_variable(name) {
					None => {
						message::error(format!("Use of undeclared variable `{}`", name), Some(self.span), Some(analyzer.context));
					},
					Some(_) => {},
				}
			},

			#[cfg(debug_assertions)]
			a => {
				message::warning(format!("No semantic analysis defined for variant of node `{}`", types::basename(a)), Some(self.span), Some(analyzer.context));
			}
		}
	}
}
