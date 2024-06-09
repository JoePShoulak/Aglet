use crate::parser::ast::Expr::*;
use crate::parser::ast::Expression;

impl Expression {
	pub fn codegen(&self) {
		match &self.node {
			FuncCall(function, arguments) => {
				match &function.node {
					Var(name) => {
						//Push all arguments onto stack?
						for arg in arguments.iter() {
							arg.codegen();
						}

						if name == "print" {
							todo!("Output logic for the print function");

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
					}
					_ => {} // Semantic analysis guarantees this will never happen
				}
			}

			Integer(_value) => {
				todo!("Push integer onto the stack")
			}

			_ => {
				todo!("No rules defined for other expression types!");
			}
		}
	}
}
