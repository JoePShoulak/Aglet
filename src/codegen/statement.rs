use crate::parser::ast::Statement;
use crate::parser::ast::Stmt::*;

use super::asm::Bytecode;

impl Statement {
	pub fn codegen(&self) -> Vec<Bytecode> {
		match &self.node {
			FuncDecl(name, _params, _return_val, program) => {
				if name.value == "main" {
					program.codegen()
				} else {
					todo!("we can't handle other functions!")
				}
			},

			ExprStmt(expr) => {
				expr.codegen()
			},

			_ => {
				todo!("AAAAA");
			},
		}
	}
}
