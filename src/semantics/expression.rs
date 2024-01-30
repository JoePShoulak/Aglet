use crate::parser::ast::Expression;
use crate::parser::ast::Expr::*;
use crate::semantics::Analyzer;
use crate::message;

impl Expression {
	fn check_binary_arithmetic(&self, analyzer: &Analyzer, type1: String, type2: String) {
		if type1 != Analyzer::INT || type2 != Analyzer::INT {
			message::error(format!("Cannot perform arithmetic on types `{}` and `{}`", type1, type2), Some(self.span), Some(analyzer.context));
		}
	}

	pub fn analyze(&self, analyzer: &mut Analyzer) -> String {
		match &self.node {
			Integer(value) => {
				//Warn if the value will not fit in 2 bytes
				if *value > 32767 {
					message::error("Value exceeds the maximum for a signed 2-byte integer (max 32767)".to_string(), Some(self.span), Some(analyzer.context));
				}

				Analyzer::INT.to_string()
			},

			Neg(expr) => {
				match &expr.node {
					//Negative integers have a different "max" than positive, by 1.
					Integer(value) => {
						if *value > 32768 {
							message::error("Value exceeds the minimum for a signed 2-byte integer (min -32768)".to_string(), Some(self.span), Some(analyzer.context));
						}

						Analyzer::INT.to_string()
					},
					_ => {
						let tp = expr.analyze(analyzer);
						if tp != Analyzer::INT {
							message::error(format!("Cannot perform arithmetic on type `{}`", tp), Some(self.span), Some(analyzer.context));
						}
						Analyzer::INT.to_string()
					},
				}
			}

			Add(a, b) => {
				let type1 = a.analyze(analyzer);
				let type2 = b.analyze(analyzer);
				self.check_binary_arithmetic(analyzer, type1, type2);
				Analyzer::INT.to_string()
			},

			Sub(a, b) => {
				let type1 = a.analyze(analyzer);
				let type2 = b.analyze(analyzer);
				self.check_binary_arithmetic(analyzer, type1, type2);
				Analyzer::INT.to_string()
			},

			Mult(a, b) => {
				let type1 = a.analyze(analyzer);
				let type2 = b.analyze(analyzer);
				self.check_binary_arithmetic(analyzer, type1, type2);
				Analyzer::INT.to_string()
			},

			Div(a, b) => {
				let type1 = a.analyze(analyzer);
				let type2 = b.analyze(analyzer);
				self.check_binary_arithmetic(analyzer, type1, type2);

				match b.node {
					Integer(value) => {
						if value == 0 {
							message::error("Division by zero".to_string(), Some(b.span), Some(analyzer.context));
						}
					},
					_ => {},
				}

				Analyzer::INT.to_string()
			},

			Mod(a, b) => {
				let type1 = a.analyze(analyzer);
				let type2 = b.analyze(analyzer);
				self.check_binary_arithmetic(analyzer, type1, type2);

				match b.node {
					Integer(value) => {
						if value == 0 {
							message::error("Division by zero".to_string(), Some(b.span), Some(analyzer.context));
						}
					},
					_ => {},
				}

				Analyzer::INT.to_string()
			},

			LessThan(a, b) => {
				let type1 = a.analyze(analyzer);
				let type2 = b.analyze(analyzer);
				self.check_binary_arithmetic(analyzer, type1, type2);
				Analyzer::INT.to_string()
			},

			LessOrEqual(a, b) => {
				let type1 = a.analyze(analyzer);
				let type2 = b.analyze(analyzer);
				self.check_binary_arithmetic(analyzer, type1, type2);
				Analyzer::INT.to_string()
			},

			GreaterThan(a, b) => {
				let type1 = a.analyze(analyzer);
				let type2 = b.analyze(analyzer);
				self.check_binary_arithmetic(analyzer, type1, type2);
				Analyzer::INT.to_string()
			},

			GreaterOrEqual(a, b) => {
				let type1 = a.analyze(analyzer);
				let type2 = b.analyze(analyzer);
				self.check_binary_arithmetic(analyzer, type1, type2);
				Analyzer::INT.to_string()
			},

			Equal(a, b) => {
				let type1 = a.analyze(analyzer);
				let type2 = b.analyze(analyzer);
				self.check_binary_arithmetic(analyzer, type1, type2);
				Analyzer::INT.to_string()
			},

			NotEqual(a, b) => {
				let type1 = a.analyze(analyzer);
				let type2 = b.analyze(analyzer);
				self.check_binary_arithmetic(analyzer, type1, type2);
				Analyzer::INT.to_string()
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
								Analyzer::VOID.to_string()
							},

							Some(func) => {
								let ct = func.param_types.len();
								if params.len() != ct {
									message::error(format!("Expected {} argument{} to function `{}`, got {}", ct, if ct == 1 {""} else {"s"}, id, params.len()), Some(self.span), Some(analyzer.context));
									message::hint(format!("Function signature is `{}{}`", id, func), Some(self.span), Some(analyzer.context));
								}

								func.return_type.clone()
							}
						}
					},

					_ => {
						message::error("Composite function names are not supported yet".to_string(), Some(name.span), Some(analyzer.context));
						Analyzer::VOID.to_string()
					}
				}
			},

			Var(name) => {
				match analyzer.get_variable(name, true) {
					None => {
						message::error(format!("Use of undeclared variable `{}`", name), Some(self.span), Some(analyzer.context));
						Analyzer::INT.to_string()
					},
					Some(var) => {
						let ret = var.data_type.clone();
						analyzer.use_variable(name);
						ret
					},
				}
			},

			Assign(variable, expr) => {
				let expr_type = expr.analyze(analyzer);
				let var_type = variable.analyze(analyzer);

				if expr_type != var_type {
					message::error(format!("Cannot assign value of type `{}` to `{}`: incompatible types", expr_type, var_type), Some(expr.span), Some(analyzer.context));
				}

				match &variable.node {
					Var(id) => {
						match analyzer.get_variable(&id, true) {
							None => {
								message::error(format!("Use of undeclared variable `{}`", id), Some(self.span), Some(analyzer.context));
								Analyzer::INT.to_string()
							},
							Some(var) => {
								if expr_type != var_type {
									message::context(var.span, analyzer.context);
									message::hint(format!("Variable `{}` was declared as type `{}` here", id, var_type), Some(var.span), Some(analyzer.context));
								}

								if !var.mutable {
									message::error(format!("Cannot assign a value to immutable variable `{}`", id), Some(self.span), Some(analyzer.context));
									message::context(var.span, analyzer.context);
									message::hint(format!("Variable `{}` was declared as immutable here", id), Some(var.span), Some(analyzer.context));
								}
								let ret = var.data_type.clone();
								analyzer.change_variable(id);
								ret
							},
						}
					},
					_ => {
						message::error("Indirect variable assignments are not supported yet".to_string(), Some(variable.span), Some(analyzer.context));
						Analyzer::INT.to_string()
					}
				}
			},

		}
	}
}
