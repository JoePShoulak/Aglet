use crate::message::Context;
use crate::parser::ast::Program;
use std::collections::HashMap;

mod program;
mod statement;
mod expression;

pub struct FuncSig {
	return_type: String,
	param_types: Vec<String>,
}

impl std::fmt::Display for FuncSig {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "({}) -> {}", self.param_types.join(", "), self.return_type)
	}
}

pub struct Scope {
	functions: HashMap<String, FuncSig>,
}

impl Scope {
	fn new() -> Scope {
		Scope {
			functions: HashMap::new(),
		}
	}
}

pub struct Analyzer<'a> {
	context: &'a Context<'a>,
	scopes: Vec<Scope>,
}

impl<'a> Analyzer<'a> {
	pub fn run(ast: &Program, context: &'a Context) -> Analyzer<'a> {
		let mut analyzer = Analyzer {
			context: context,
			scopes: vec![Scope::new()],
		};

		analyzer.set_function(&String::from("print"), vec![String::from("int")], &String::from("void"));

		ast.analyze(&mut analyzer);
		analyzer
	}

	pub fn push_scope(&mut self) {
		self.scopes.push(Scope::new());
	}

	pub fn pop_scope(&mut self) {
		self.scopes.pop();
	}

	pub fn get_function(&self, name: &String) -> Option<&FuncSig> {
		for scope in &self.scopes {
			let func = scope.functions.get(name);
			match func {
				None => {},
				_ => { return func; },
			}
		}

		return None;
	}

	pub fn set_function(&mut self, name: &String, params: Vec<String>, return_type: &String) {
		let scope = self.scopes.last_mut().unwrap();
		scope.functions.insert(name.to_string(), FuncSig {
			return_type: return_type.to_string(),
			param_types: params,
		});
	}

	pub fn valid_return_type(&self, return_type: &String) -> bool {
		["int", "void"].iter().any(|&s| s == return_type)
	}

	pub fn valid_data_type(&self, data_type: &String) -> bool {
		["int"].iter().any(|&s| s == data_type)
	}
}
