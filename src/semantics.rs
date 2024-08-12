use crate::flags::Options;
use crate::lexer::Span;
use crate::message::Context;
use crate::parser::ast::Program;
use std::collections::HashMap;

mod assign;
mod expression;
mod program;
mod statement;

#[derive(Clone)]
pub enum DataType {
	FuncSig(String, Vec<String>),
	VarSig(String),
}

pub struct TypeSignature {
	data_type: DataType,
	mutable: bool,
	span: Span,
	used: i64,
	changed: i64,
}

impl std::fmt::Display for TypeSignature {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.data_type)
	}
}

impl std::fmt::Display for DataType {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match &self {
			DataType::FuncSig(return_type, param_types) => {
				write!(f, "({}) -> {}", param_types.join(", "), return_type)
			}
			DataType::VarSig(value) => write!(f, "{}", value),
		}
	}
}

pub struct Scope {
	variables: HashMap<String, TypeSignature>,
}

impl Scope {
	fn new() -> Scope {
		Scope {
			variables: HashMap::new(),
		}
	}
}

pub struct Analyzer<'a> {
	context: &'a Context<'a>,
	scopes: Vec<Scope>,
	func_stack: Vec<String>,
	loops: i64,
	flags: &'a Options,
}

impl<'a> Analyzer<'a> {
	const INT: &'static str = "int";
	const VOID: &'static str = "void";
	const FUNC_MAIN: &'static str = "main";

	pub fn run(ast: &Program, context: &'a Context, flags: &'a Options) -> Analyzer<'a> {
		let mut analyzer = Analyzer {
			context: context,
			scopes: vec![Scope::new()],
			func_stack: vec![],
			loops: 0,
			flags: flags,
		};

		analyzer.set_function(
			&String::from("print"),
			vec![Analyzer::INT.to_string()],
			Analyzer::VOID,
			Span { lo: 0, hi: 0 },
		);

		ast.analyze(&mut analyzer);
		analyzer
	}

	pub fn push_scope(&mut self) {
		self.scopes.push(Scope::new());
	}

	pub fn pop_scope(&mut self) -> Scope {
		self.scopes.pop().unwrap()
	}

	pub fn get_current_function(&self) -> Option<(&DataType, &String)> {
		match self.func_stack.last() {
			None => None,
			Some(func) => Some((&self.get_variable(func, true).unwrap().data_type, func)),
		}
	}

	pub fn set_function(
		&mut self,
		name: &String,
		params: Vec<String>,
		return_type: &str,
		span: Span,
	) {
		let scope = self.scopes.last_mut().unwrap();
		scope.variables.insert(
			name.to_string(),
			TypeSignature {
				data_type: DataType::FuncSig(return_type.to_string(), params),
				mutable: false,
				span: Span {
					lo: span.lo,
					hi: span.hi,
				},
				used: 0,
				changed: 0,
			},
		);
	}

	pub fn valid_return_type(&self, return_type: &String) -> bool {
		["int", "void"].iter().any(|&s| s == return_type)
	}

	pub fn get_variable(&self, name: &String, all_scopes: bool) -> Option<&TypeSignature> {
		if all_scopes {
			for scope in &self.scopes {
				let var = scope.variables.get(name);
				match var {
					None => {}
					_ => {
						return var;
					}
				}
			}
			None
		} else {
			self.scopes.last().unwrap().variables.get(name)
		}
	}

	pub fn set_variable(&mut self, name: &String, data_type: &str, mutable: bool, span: Span) {
		let scope = self.scopes.last_mut().unwrap();
		scope.variables.insert(
			name.to_string(),
			TypeSignature {
				data_type: DataType::VarSig(data_type.to_string()),
				mutable: mutable,
				span: Span {
					lo: span.lo,
					hi: span.hi,
				},
				used: 0,
				changed: 0,
			},
		);
	}

	pub fn set_entity(&mut self, name: &String, data_type: &DataType, mutable: bool, span: Span) {
		let scope = self.scopes.last_mut().unwrap();
		scope.variables.insert(
			name.to_string(),
			TypeSignature {
				data_type: data_type.clone(),
				mutable: mutable,
				span: Span {
					lo: span.lo,
					hi: span.hi,
				},
				used: 0,
				changed: 0,
			},
		);
	}

	pub fn change_variable(&mut self, name: &String) {
		for scope in &mut self.scopes {
			let var = scope.variables.get_mut(name);
			match var {
				None => {}
				Some(value) => {
					value.changed += 1;
				}
			}
		}
	}

	pub fn use_variable(&mut self, name: &String) {
		for scope in &mut self.scopes {
			let var = scope.variables.get_mut(name);
			match var {
				None => {}
				Some(value) => {
					value.used += 1;
				}
			}
		}
	}

	pub fn valid_data_type(&self, data_type: &String) -> bool {
		["void"].iter().any(|&s| s != data_type)
	}
}
