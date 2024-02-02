use crate::parser::ast::Expression;
use crate::parser::ast::Expr::*;
use crate::semantics::Analyzer;
use crate::message;

impl Expression {
	pub fn analyze_assign(&self, analyzer: &mut Analyzer, variable: &Expression, expr: &Expression) -> String {
		let expr_type = expr.analyze(analyzer);
		let var_type = variable.analyze(analyzer);

		if expr_type != var_type {
			message::error(format!("Cannot mutate value of type `{}` to `{}`: incompatible types", expr_type, var_type), Some(expr.span), Some(analyzer.context));
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
							message::error(format!("Cannot mutate immutable variable `{}`", id), Some(self.span), Some(analyzer.context));
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
	}
}