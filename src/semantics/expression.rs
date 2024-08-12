use crate::message;
use crate::parser::ast::Expr::*;
use crate::parser::ast::Expression;
use crate::semantics::Analyzer;

use super::DataType;

impl Expression {
	fn check_binary_arithmetic(&self, analyzer: &Analyzer, type1: String, type2: String) {
		if type1 != Analyzer::INT || type2 != Analyzer::INT {
			message::error(
				format!(
					"Cannot perform arithmetic on types `{}` and `{}`",
					type1, type2
				),
				Some(self.span),
				Some(analyzer.context),
			);
		}
	}

	pub fn analyze(&self, analyzer: &mut Analyzer) -> DataType {
		match &self.node {
			Integer(value) => {
				//Warn if the value will not fit in 2 bytes
				if *value > 32767 {
					message::error(
						"Value exceeds the maximum for a signed 2-byte integer (max 32767)"
							.to_string(),
						Some(self.span),
						Some(analyzer.context),
					);
				}

				DataType::VarSig(Analyzer::INT.to_string())
			}

			Neg(expr) => {
				match &expr.node {
					//Negative integers have a different "max" than positive, by 1.
					Integer(value) => {
						if *value > 32768 {
							message::error("Value exceeds the minimum for a signed 2-byte integer (min -32768)".to_string(), Some(self.span), Some(analyzer.context));
						}

						DataType::VarSig(Analyzer::INT.to_string())
					}
					_ => {
						let tp = format!("{}", expr.analyze(analyzer));
						if tp != Analyzer::INT {
							message::error(
								format!("Cannot perform arithmetic on type `{}`", tp),
								Some(self.span),
								Some(analyzer.context),
							);
						}
						DataType::VarSig(Analyzer::INT.to_string())
					}
				}
			}

			Add(a, b) => {
				let type1 = format!("{}", a.analyze(analyzer));
				let type2 = format!("{}", b.analyze(analyzer));
				self.check_binary_arithmetic(analyzer, type1, type2);
				DataType::VarSig(Analyzer::INT.to_string())
			}

			Sub(a, b) => {
				let type1 = format!("{}", a.analyze(analyzer));
				let type2 = format!("{}", b.analyze(analyzer));
				self.check_binary_arithmetic(analyzer, type1, type2);
				DataType::VarSig(Analyzer::INT.to_string())
			}

			Mult(a, b) => {
				let type1 = format!("{}", a.analyze(analyzer));
				let type2 = format!("{}", b.analyze(analyzer));
				self.check_binary_arithmetic(analyzer, type1, type2);
				DataType::VarSig(Analyzer::INT.to_string())
			}

			Div(a, b) => {
				let type1 = format!("{}", a.analyze(analyzer));
				let type2 = format!("{}", b.analyze(analyzer));
				self.check_binary_arithmetic(analyzer, type1, type2);

				match b.node {
					Integer(value) => {
						if value == 0 {
							message::error(
								"Division by zero".to_string(),
								Some(b.span),
								Some(analyzer.context),
							);
						}
					}
					_ => {}
				}

				DataType::VarSig(Analyzer::INT.to_string())
			}

			Mod(a, b) => {
				let type1 = format!("{}", a.analyze(analyzer));
				let type2 = format!("{}", b.analyze(analyzer));
				self.check_binary_arithmetic(analyzer, type1, type2);

				match b.node {
					Integer(value) => {
						if value == 0 {
							message::error(
								"Division by zero".to_string(),
								Some(b.span),
								Some(analyzer.context),
							);
						}
					}
					_ => {}
				}

				DataType::VarSig(Analyzer::INT.to_string())
			}

			LessThan(a, b) => {
				let type1 = format!("{}", a.analyze(analyzer));
				let type2 = format!("{}", b.analyze(analyzer));
				self.check_binary_arithmetic(analyzer, type1, type2);
				DataType::VarSig(Analyzer::INT.to_string())
			}

			LessOrEqual(a, b) => {
				let type1 = format!("{}", a.analyze(analyzer));
				let type2 = format!("{}", b.analyze(analyzer));
				self.check_binary_arithmetic(analyzer, type1, type2);
				DataType::VarSig(Analyzer::INT.to_string())
			}

			GreaterThan(a, b) => {
				let type1 = format!("{}", a.analyze(analyzer));
				let type2 = format!("{}", b.analyze(analyzer));
				self.check_binary_arithmetic(analyzer, type1, type2);
				DataType::VarSig(Analyzer::INT.to_string())
			}

			GreaterOrEqual(a, b) => {
				let type1 = format!("{}", a.analyze(analyzer));
				let type2 = format!("{}", b.analyze(analyzer));
				self.check_binary_arithmetic(analyzer, type1, type2);
				DataType::VarSig(Analyzer::INT.to_string())
			}

			Equal(a, b) => {
				let type1 = format!("{}", a.analyze(analyzer));
				let type2 = format!("{}", b.analyze(analyzer));
				self.check_binary_arithmetic(analyzer, type1, type2);
				DataType::VarSig(Analyzer::INT.to_string())
			}

			NotEqual(a, b) => {
				let type1 = format!("{}", a.analyze(analyzer));
				let type2 = format!("{}", b.analyze(analyzer));
				self.check_binary_arithmetic(analyzer, type1, type2);
				DataType::VarSig(Analyzer::INT.to_string())
			}

			FuncCall(name, params) => match &name.node {
				Var(id) => {
					for param in params.iter() {
						param.analyze(analyzer);
					}

					let return_type = match analyzer.get_variable(id, true) {
						None => {
							message::error(
								format!("Use of undeclared function `{}`", id),
								Some(name.span),
								Some(analyzer.context),
							);
							DataType::VarSig(Analyzer::VOID.to_string())
						}

						Some(func) => match &func.data_type {
							DataType::FuncSig(return_type, param_types) => {
								let ct = param_types.len();
								if params.len() != ct {
									message::error(
										format!(
											"Expected {} argument{} to function `{}`, got {}",
											ct,
											if ct == 1 { "" } else { "s" },
											id,
											params.len()
										),
										Some(self.span),
										Some(analyzer.context),
									);
									message::hint(
										format!("Function signature is `{}{}`", id, func),
										Some(self.span),
										Some(analyzer.context),
									);
								}

								DataType::VarSig(return_type.clone())
							}

							DataType::VarSig(value) => {
								message::error(
									format!("Cannot call `{}` because it is not a function", id,),
									Some(self.span),
									Some(analyzer.context),
								);
								message::context(func.span, analyzer.context);
								message::hint(
									format!(
										"Variable `{}` was declared as type `{}` here",
										id, value
									),
									Some(func.span),
									Some(analyzer.context),
								);

								DataType::VarSig(Analyzer::VOID.to_string())
							}
						},
					};

					analyzer.use_variable(id);
					return_type
				}

				_ => {
					message::error(
						"Composite function names are not supported yet".to_string(),
						Some(name.span),
						Some(analyzer.context),
					);
					DataType::VarSig(Analyzer::VOID.to_string())
				}
			},

			Var(name) => match analyzer.get_variable(name, true) {
				None => {
					message::error(
						format!("Use of undeclared variable `{}`", name),
						Some(self.span),
						Some(analyzer.context),
					);
					DataType::VarSig(Analyzer::INT.to_string())
				}
				Some(var) => {
					if !var.mutable && analyzer.flags.language_server {
						message::diagnostic(
							message::DiagnosticType::Constant,
							Some(self.span),
							Some(analyzer.context),
						);
					}

					let return_type = var.data_type.clone();
					analyzer.use_variable(name);
					return_type
				}
			},

			FuncDeclAnonymous(params, expr) => {
				analyzer.push_scope();
				analyzer.func_stack.push("anonymous function".to_string());

				//Declare variables in scope. We may want to allow them to be mutable? For now they are immutable
				for param in params.iter() {
					if analyzer.flags.language_server {
						message::diagnostic(
							message::DiagnosticType::Constant,
							Some(param.name.span),
							Some(analyzer.context),
						);
					}

					analyzer.set_variable(
						&param.name.value,
						&param.datatype.value,
						false,
						param.span,
					);
				}

				let expr_type = expr.analyze(analyzer);
				analyzer.func_stack.pop();
				let scope = analyzer.pop_scope();

				//Check for any mutable variables that don't have to be
				for (name, signature) in scope.variables {
					if signature.used == 0 && !name.starts_with("_") {
						if !analyzer.flags.warn_suppress {
							message::warning(format!("Variable `{name}` is never used. If this is intentional, prefix the variable name with an underscore (e.g. `_{name}`)"), Some(signature.span), Some(analyzer.context));
						}
					}
					if signature.mutable && signature.changed == 0 {
						if !analyzer.flags.warn_suppress {
							message::warning(format!("Variable `{name}` does not need to be mutable. Consider replacing `let` with `set`"), Some(signature.span), Some(analyzer.context));
						}
					}
				}

				DataType::FuncSig(
					format!("{}", expr_type),
					params.iter().map(|p| p.datatype.value.clone()).collect(),
				)
			}
		}
	}
}
