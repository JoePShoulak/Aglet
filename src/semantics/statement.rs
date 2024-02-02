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

	pub fn analyze(&self, analyzer: &mut Analyzer) -> bool {
		//Make sure everything is in the correct scope
		match &self.node {
			FuncDecl(_, _, _, _) => {
				if analyzer.func_stack.len() > 0 {
					message::error("Functions cannot be declared inside other functions".to_string(), Some(self.span), Some(analyzer.context));
				}
			},
			_ => {
				if analyzer.func_stack.len() == 0 {
					message::error("This statement must be inside a function".to_string(), Some(self.span), Some(analyzer.context));
					return false;
				}
			},
		}


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

				//Analyze the function body
				analyzer.push_scope();
				analyzer.func_stack.push(name.value.clone());

				//Declare variables in scope. We may want to allow them to be mutable? For now they are immutable
				for param in params.iter() {
					analyzer.set_variable(&param.name, &param.datatype, false, param.span);
				}

				let return_guaranteed = body.analyze(analyzer);

				analyzer.func_stack.pop();
				let scope = analyzer.pop_scope();

				//Check for any mutable variables that don't have to be
				for (name, signature) in scope.variables {
					if signature.used == 0 && !name.starts_with("_") {
						message::warning(format!("Variable `{name}` is never used. If this is intentional, prefix the variable name with an underscore (e.g. `_{name}`)"), Some(signature.span), Some(analyzer.context));
					}
					if signature.mutable && signature.changed == 0 {
						message::warning(format!("Variable `{name}` does not need to be mutable. Consider replacing `let` with `set`"), Some(signature.span), Some(analyzer.context));
					}
				}

				if return_type.value != Analyzer::VOID  && !return_guaranteed {
					message::error(format!("Function `{}` might not return a value. A value of type `{}` must always be returned", name.value, return_type.value), Some(self.span), Some(analyzer.context));
				}
			},

			VarDecl(qualifiers, name, datatype, value) => {
				let deduced_type = value.analyze(analyzer);

				match analyzer.get_variable(&name.value, false) {
					None => {},
					Some(var) => {
						//Do we want to allow redeclaration of variables in the same scope? Disallow for now.
						message::error(format!("Redeclaration of variable `{}`", name.value), Some(name.span), Some(analyzer.context));
						message::hint(format!("Variable `{}` declared here", name.value), Some(name.span), Some(analyzer.context));

						message::context(var.span, analyzer.context);
						message::hint("But it was already declared here".to_string(), Some(var.span), Some(analyzer.context));
						return false;
					},
				}

				let mutable = match qualifiers[0] {
					Mutable => true,
					Immutable => false,
				};

				match **datatype {
					None => {
						//Deduce the type from the expression.

						if !analyzer.valid_data_type(&deduced_type) {
							message::error(format!("Cannot assign `{}` value to variable `{}`: invalid data type", deduced_type, name.value), Some(value.span), Some(analyzer.context));
							self.hint_function_signature(value, analyzer);
						}
						analyzer.set_variable(&name.value, &deduced_type, mutable, name.span);
					},
					Some(ref data_type) => {
						if !analyzer.valid_data_type(&data_type.value) {
							message::error(format!("Unknown data type `{}`. Only `{}` is supported at this time", data_type.value, Analyzer::INT), Some(data_type.span), Some(analyzer.context));
						} else if deduced_type != data_type.value {
							message::error(format!("Cannot assign `{}` value to variable `{}` of type `{}`: incompatible types", deduced_type, name.value, data_type.value), Some(value.span), Some(analyzer.context));
							self.hint_function_signature(value, analyzer);
						}

						analyzer.set_variable(&name.value, &data_type.value, mutable, name.span);
					},
				}
			},

			ReturnStmt(expr) => {
				//At this point, we know that return statements will be inside a function.
				let (func, name) = analyzer.get_current_function().unwrap();

				match **expr {
					None => {
						if func.return_type != Analyzer::VOID {
							message::error(format!("Return statement must have a value"), Some(self.span), Some(analyzer.context));
							message::hint(format!("Function `{}` requires a return value of type `{}`", name, func.return_type), None, None);
						}
					},
					Some(ref expr) => {
						if func.return_type == Analyzer::VOID {
							message::error(format!("Return statement cannot have a value"), Some(self.span), Some(analyzer.context));
							message::hint(format!("Function `{}` does not return anything", name), None, None);
						} else {
							let expr_type = expr.analyze(analyzer);
							if expr_type == Analyzer::VOID {
								message::error("Expression does not return a value".to_string(), Some(expr.span), Some(analyzer.context));
								self.hint_function_signature(expr, analyzer);
							}
						}
					}
				}

				return true;
			}

			IfStmt(condition, stmts_true, stmts_false) => {
				let expr_type = condition.analyze(analyzer);
				if expr_type == Analyzer::VOID {
					message::error("Expression does not return a value".to_string(), Some(condition.span), Some(analyzer.context));
					self.hint_function_signature(condition, analyzer);
				}

				//If statements are only guaranteed to return if all the branches are also guaranteed to return.
				return stmts_true.analyze(analyzer) && stmts_false.analyze(analyzer);
			}

		}

		return false;
	}
}
