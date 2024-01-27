use crate::message::Context;
use crate::parser::ast::Program;

mod program;
mod statement;
mod expression;

pub struct Analyzer<'a> {
	context: &'a Context<'a>,
}

impl<'a> Analyzer<'a> {
	pub fn run(ast: &Program, context: &'a Context) -> Analyzer<'a> {
		let mut analyzer = Analyzer {
			context: context,
		};
		ast.analyze(&mut analyzer);
		analyzer
	}
}
