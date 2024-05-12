use crate::parser::ast::Expression;
use crate::parser::ast::Expr::*;

use super::asm::Bytecode;
use super::asm::Bytecode::*;

impl Expression {
	pub fn codegen(&self) -> Vec<Bytecode> {
		match &self.node {
			FuncCall(function, arguments) => {
				match &function.node {
					Var(name) => {
						//Push all arguments onto stack?
						let mut bc: Vec<Bytecode> = arguments.iter().flat_map(|arg| arg.codegen()).collect();

						if name == "print" {
							// todo!("Output logic for the print function");

							bc.push(LDA("low_byte_of_int".to_string()));
							bc.push(STA("MATH_CONVERT_VAL".to_string()));
							bc.push(LDA("high_byte_of_int".to_string()));
							bc.push(STA("MATH_CONVERT_VAL + 1".to_string()));
							bc.push(JSR("MATH_int_to_string".to_string()));
							/* js psuedo
							return `
								lda ${low byte of our int}
								sta MATH_CONVERT_VAL
								lda ${high byte of our int}
								sta MATH_CONVERT_VAL + 1
								jsr MATH_int_to_string

								lda #<MATH_CONVERT_OUT
								sta LCD_STRING_PTR
								lda #>MATH_CONVERT_OUT
								sta LCD_STRING_PTR + 1
								jsr LCD_print_string
							`
							*/
						} else {
							todo!("We can't handle other functions!");
						}

						bc
					},
					_ => {
						panic!("COMPILER BUG: Invalid node in function call!");
					}, // Semantic analysis guarantees this will never happen
				}
			},

			Integer(_value) => {
				todo!("Push integer onto the stack")
			},

			_ => {
				todo!("No rules defined for other expression types!");
			}
		}
	}
}
