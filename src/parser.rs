pub mod ast {
	use crate::lexer::Span;
	use crate::lexer::Token;

	#[derive(Debug)]
	pub struct Program {
		pub stmts: Vec<Statement>,
	}

	#[derive(Debug)]
	pub struct Statement {
		pub span: Span,
		pub node: Stmt,
	}

	#[derive(Debug)]
	pub enum Stmt {
		ExprStmt(Box<Expression>),
		FuncDecl(Box<Ident>, Box<Vec<Param>>, Box<Ident>, Box<Program>),
		ReturnStmt(Box<Option<Expression>>),
		IfStmt(Box<Expression>, Box<Program>, Box<Program>),
		VarDecl(Box<Vec<Token>>, Box<String>, Box<Option<String>>, Box<Expression>),
	}

	#[derive(Debug)]
	pub struct Expression {
		pub span: Span,
		pub node: Expr,
	}

	#[derive(Debug)]
	pub enum Expr {
		//Arithmetic
		Neg(Box<Expression>),
		Add(Box<Expression>, Box<Expression>),
		Sub(Box<Expression>, Box<Expression>),
		Mult(Box<Expression>, Box<Expression>),
		Div(Box<Expression>, Box<Expression>),
		Mod(Box<Expression>, Box<Expression>),

		//Boolean comparison
		LessThan(Box<Expression>, Box<Expression>),
		LessOrEqual(Box<Expression>, Box<Expression>),
		GreaterThan(Box<Expression>, Box<Expression>),
		GreaterOrEqual(Box<Expression>, Box<Expression>),
		Equal(Box<Expression>, Box<Expression>),
		NotEqual(Box<Expression>, Box<Expression>),

		//Misc
		Assign(Box<Expression>, Box<Expression>),

		Var(String),
		Integer(i64),
		FuncCall(Box<Expression>, Box<Vec<Expression>>),
	}

	#[derive(Debug)]
	pub struct ParamList {
		pub params: Vec<Expression>,
	}

	#[derive(Debug)]
	pub struct ParamDeclList {
		pub params: Vec<Param>,
	}

	#[derive(Debug)]
	pub struct Param {
		pub span: Span,
		pub name: String,
		pub datatype: String,
	}

	#[derive(Debug)]
	pub struct Ident {
		pub span: Span,
		pub value: String,
	}
}

use ast::*;
use crate::lexer::*;
use crate::lexer::Token::*;
use plex::parser;

parser! {
	fn parse_(Token, Span);

	//Combine two spans.
	(a, b) {
		Span {
			lo: a.lo,
			hi: b.hi,
		}
	}

	program: Program {
		statements[s] => Program { stmts: s }
	}

	statements: Vec<Statement> {
		=> vec![],
		statements[mut st] statement[e] => {
			st.push(e);
			st
		}
	}

	statement: Statement {
		assign[e] Semicolon => Statement {
			span: span!(),
			node: Stmt::ExprStmt(Box::new(e)),
		},

		KwdFunction ident[name] LParen RParen Arrow ident[return_type] LBrace program[p] RBrace => Statement {
			span: span!(),
			node: Stmt::FuncDecl(Box::new(name), Box::new(vec![]), Box::new(return_type), Box::new(p)),
		},

		KwdFunction ident[name] LParen param_decl_list[params] RParen Arrow ident[return_type] LBrace program[p] RBrace => Statement {
			span: span!(),
			node: Stmt::FuncDecl(Box::new(name), Box::new(params), Box::new(return_type), Box::new(p)),
		},

		KwdReturn assign[e] Semicolon => Statement {
			span: span!(),
			node: Stmt::ReturnStmt(Box::new(Some(e))),
		},

		KwdReturn Semicolon => Statement {
			span: span!(),
			node: Stmt::ReturnStmt(Box::new(None)),
		},

		KwdIf assign[e] LBrace program[p] RBrace KwdElse LBrace program[p2] RBrace => Statement {
			span: span!(),
			node: Stmt::IfStmt(Box::new(e), Box::new(p), Box::new(p2)),
		},

		KwdIf assign[e] LBrace program[p] RBrace => Statement {
			span: span!(),
			node: Stmt::IfStmt(Box::new(e), Box::new(p), Box::new(Program{stmts: vec![]})),
		},

		//Variable declaration without a specified type.
		qualifiers[q] Identifier(name) OperAssign assign[e] Semicolon => Statement {
			span: span!(),
			node: Stmt::VarDecl(Box::new(q), Box::new(name), Box::new(None), Box::new(e)),
		},

		//Variable declaration WITH a specified type.
		qualifiers[q] Identifier(name) Colon Identifier(typename) OperAssign assign[e] Semicolon => Statement {
			span: span!(),
			node: Stmt::VarDecl(Box::new(q), Box::new(name), Box::new(Some(typename)), Box::new(e)),
		},
	}

	ident: Ident {
		Identifier(value) => Ident {
			span: span!(),
			value: value,
		}
	}

	//Variable qualifiers are an array, just in case we want to allow multiple quals on var decls in the future.
	qualifiers: Vec<Token> {
		KwdConstant => vec![KwdConstant],
		KwdMutable => vec![KwdMutable],
	}

	param_decl_list: Vec<Param> {
		param_decl_list[mut lhs] Comma param_decl[rhs] => {
			lhs.push(rhs);
			lhs
		},
		param_decl[a] => vec![a],
	}

	param_decl: Param {
		Identifier(name) Colon Identifier(datatype) => Param {
			span: span!(),
			name: name,
			datatype: datatype,
		}
	}

	//Assignment (lowest precedence)
	assign: Expression {
		assign[lhs] OperAssign compare[rhs] => Expression {
			span: span!(),
			node: Expr::Assign(Box::new(lhs), Box::new(rhs)),
		},
		compare[x] => x,
	}

	//Boolean comparison (lower precedence than addition)
	compare: Expression {
		compare[lhs] OperLessThan term[rhs] => Expression {
			span: span!(),
			node: Expr::LessThan(Box::new(lhs), Box::new(rhs)),
		},
		compare[lhs] OperLessOrEqual term[rhs] => Expression {
			span: span!(),
			node: Expr::LessOrEqual(Box::new(lhs), Box::new(rhs)),
		},
		compare[lhs] OperGreaterThan term[rhs] => Expression {
			span: span!(),
			node: Expr::GreaterThan(Box::new(lhs), Box::new(rhs)),
		},
		compare[lhs] OperGreaterOrEqual term[rhs] => Expression {
			span: span!(),
			node: Expr::GreaterOrEqual(Box::new(lhs), Box::new(rhs)),
		},
		compare[lhs] OperEqual term[rhs] => Expression {
			span: span!(),
			node: Expr::Equal(Box::new(lhs), Box::new(rhs)),
		},
		compare[lhs] OperNotEqual term[rhs] => Expression {
			span: span!(),
			node: Expr::NotEqual(Box::new(lhs), Box::new(rhs)),
		},
		term[x] => x,
	}


	//Addition (lower precedence than multiplication)
	term: Expression {
		term[lhs] OperPlus factor[rhs] => Expression {
			span: span!(),
			node: Expr::Add(Box::new(lhs), Box::new(rhs)),
		},
		term[lhs] OperMinus factor[rhs] => Expression {
			span: span!(),
			node: Expr::Sub(Box::new(lhs), Box::new(rhs)),
		},
		factor[x] => x,
	}

	//Multiplication
	factor: Expression {
		factor[lhs] OperMult atom[rhs] => Expression {
			span: span!(),
			node: Expr::Mult(Box::new(lhs), Box::new(rhs)),
		},
		factor[lhs] OperDiv atom[rhs] => Expression {
			span: span!(),
			node: Expr::Div(Box::new(lhs), Box::new(rhs)),
		},
		factor[lhs] OperMod atom[rhs] => Expression {
			span: span!(),
			node: Expr::Mod(Box::new(lhs), Box::new(rhs)),
		},

		OperMinus atom[e] => Expression {
			span: span!(),
			node: Expr::Neg(Box::new(e)),
		},

		atom[x] => x,
	}

	//AST rules for any node that can be a single value in an expression.
	atom: Expression {
		Identifier(i) => Expression {
			span: span!(),
			node: Expr::Var(i),
		},

		Integer(i) => Expression {
			span: span!(),
			node: Expr::Integer(i),
		},

		atom[lhs] LParen param_list[rhs] RParen => Expression {
			span: span!(),
			node: Expr::FuncCall(Box::new(lhs), Box::new(rhs)),
		},

		atom[lhs] LParen RParen => Expression {
			span: span!(),
			node: Expr::FuncCall(Box::new(lhs), Box::new(vec![])),
		},

		LParen assign[a] RParen => a,
	}

	param_list: Vec<Expression> {
		param_list[mut lhs] Comma assign[rhs] => {
			lhs.push(rhs);
			lhs
		},
		assign[a] => vec![a],
	}
}

pub fn parse<I: Iterator<Item = (Token, Span)>>(i: I) -> Result<Program, (Option<(Token, Span)>, &'static str)> {
	parse_(i)
}

#[cfg(debug_assertions)] use regex::Regex;
#[cfg(debug_assertions)] use colored::Colorize;
#[cfg(debug_assertions)]
pub fn pretty(ast: &Program) -> String {
	let fluff = Regex::new(r"\n *[\)\}\]],?").unwrap();
	let spans = Regex::new(r"\n *(lo|hi)").unwrap();
	let other = Regex::new(r"((Literal|Var)\()\n *([^\n]+)").unwrap();

	let text = format!("{:#?}", ast).replace("    ", "  ");

	let s1 = fluff.replace_all(&text, "");
	let s2 = spans.replace_all(&s1, " $1");
	let s3 = other.replace_all(&s2, "$1 $3".bold().yellow().to_string());

	return s3.to_string();
}
