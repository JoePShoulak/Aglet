pub mod ast {
	use crate::lexer::Span;

	#[derive(Debug)]
	pub struct Program {
		pub stmts: Vec<Expression>,
	}

	#[derive(Debug)]
	pub struct Expression {
		pub span: Span,
		pub node: Expr,
	}

	#[derive(Debug)]
	pub enum Expr {
		//Arithmetic
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

		Var(String),
		Literal(i64),
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

	statements: Vec<Expression> {
		=> vec![],
		statements[mut st] compare[e] Semicolon =>  {
			st.push(e);
			st
		}
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
			node: Expr::Literal(i),
		},

		LParen term[a] RParen => a,
	}
}

pub fn parse<I: Iterator<Item = (Token, Span)>>(i: I) -> Result<Program, (Option<(Token, Span)>, &'static str)> {
	parse_(i)
}
