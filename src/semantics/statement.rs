use crate::parser::ast::Statement;
use crate::parser::ast::Expression;
use crate::parser::ast::Stmt::*;
use crate::parser::ast::Expr::*;
use crate::parser::ast::Qualifier::*;
use crate::semantics::Analyzer;
use crate::message;

impl Statement {
	fn hint_function_signature(&self, expr: &Expression, analyzer: &Analyzer) {
		//If the expression is a function, try to print a hint about its signature.
		match &expr.node {
			FuncCall(name, _) => {
				match &name.node {
					Var(id) => {
						match analyzer.get_function(id) {
							None => {},
							Some(func) => {
								message::hint(format!("Function signature is `{}{}`", id, func), Some(expr.span), Some(analyzer.context));
							},
						}
					},
					_ => {},
				}
			},
			_ => {},
		}
	}

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
						analyzer.set_function(&name.value, params, &return_type.value.clone());
					},
				}

				if !analyzer.valid_return_type(&return_type.value) {
					message::error(format!("Unknown return type `{}`. Valid types are `{}` or `{}`", return_type.value, Analyzer::INT, Analyzer::VOID), Some(return_type.span), Some(analyzer.context));
				}

				if name.value == "main" {
					//Force the main() function to have a specific signature
					if params.len() > 0 || return_type.value != Analyzer::VOID {
						let span = if params.len() > 0 && return_type.value != Analyzer::VOID {
							crate::lexer::Span {
								lo: params[0].span.lo,
								hi: return_type.span.hi,
							}
						} else if params.len() > 0 {
							crate::lexer::Span {
								lo: params[0].span.lo,
								hi: params[params.len() - 1].span.hi,
							}
						} else {
							return_type.span
						};

						message::error(format!("Function signature for `main` must be `() -> {}`", Analyzer::VOID), Some(span), Some(analyzer.context));
					}

					//Also only allow it to be declared in the global scope
					if analyzer.scopes.len() > 1 {
						message::error(format!("Function `main` may only be declared in the global scope"), Some(name.span), Some(analyzer.context));
					}
				}

				if body.stmts.len() > 0 {
					analyzer.push_scope();

					//Declare variables in scope. We may want to allow them to be mutable? For now they are immutable
					for param in params.iter() {
						analyzer.set_variable(&param.name, &param.datatype, false, param.span);
					}

					body.analyze(analyzer);
					analyzer.pop_scope();
				}
			},

			VarDecl(qualifiers, name, datatype, value) => {
				value.analyze(analyzer);

				match **datatype {
					None => {
						//In the future, type deduction would be nice. For now, throw an error.
						message::error(format!("Variable `{}` must have a type", name.value), Some(name.span), Some(analyzer.context));
					},
					Some(ref data_type) => {
						if !analyzer.valid_data_type(&data_type.value) {
							message::error(format!("Unknown data type `{}`. Only `{}` is supported at this time", data_type.value, Analyzer::INT), Some(data_type.span), Some(analyzer.context));
						}

						match analyzer.get_variable(&name.value, false) {
							None => {},
							Some(var) => {
								//Do we want to allow redeclaration of variables in the same scope? Disallow for now.
								message::error(format!("Redeclaration of variable `{}`", name.value), Some(name.span), Some(analyzer.context));
								message::hint(format!("Variable `{}` declared here", name.value), Some(name.span), Some(analyzer.context));

								message::context(var.span, analyzer.context);
								message::hint("But it was already declared here".to_string(), Some(var.span), Some(analyzer.context));
							},
						}

						let mutable = match qualifiers[0] {
							Mutable => true,
							Immutable => false,
						};

						analyzer.set_variable(&name.value, &data_type.value, mutable, name.span);
					},
				}
			},

			ReturnStmt(expr) => {
				match **expr {
					None => {},
					Some(ref expr) => {
						let expr_type = expr.analyze(analyzer);
						if expr_type == Analyzer::VOID {
							message::error("Expression does not return a value".to_string(), Some(expr.span), Some(analyzer.context));
							self.hint_function_signature(expr, analyzer);
						}
					}
				}
			}

			IfStmt(condition, stmts_true, stmts_false) => {
				let expr_type = condition.analyze(analyzer);
				if expr_type == Analyzer::VOID {
					message::error("Expression does not return a value".to_string(), Some(condition.span), Some(analyzer.context));
					self.hint_function_signature(condition, analyzer);
				}

				for stmt in &stmts_true.stmts {
					stmt.analyze(analyzer);
				}

				for stmt in &stmts_false.stmts {
					stmt.analyze(analyzer);
				}
			}

		}
	}
}
