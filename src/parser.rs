pub mod ast {
	use crate::lexer::Span;

	#[derive(Debug)]
	pub struct Program {
		pub stmts: Vec<Expression>,
	}

	#[derive(Debug)]
	pub struct Expression {
		pub span: Span,
		pub node: Expr_,
	}

	#[derive(Debug)]
	pub enum Expr_ {
		Add(Box<Expression>, Box<Expression>),
		Sub(Box<Expression>, Box<Expression>),
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
		statements[mut st] term[e] Semicolon =>  {
			st.push(e);
			st
		}
	}

	//AST rules for creating addition token.
	term: Expression {
		term[lhs] OperPlus atom[rhs] => Expression {
			span: span!(),
			node: Expr_::Add(Box::new(lhs), Box::new(rhs)),
		},
		term[lhs] OperMinus atom[rhs] => Expression {
			span: span!(),
			node: Expr_::Sub(Box::new(lhs), Box::new(rhs)),
		},
		atom[x] => x,
	}

	//AST rules for any node that can be a single value in an expression.
	atom: Expression {
		Identifier(i) => Expression {
			span: span!(),
			node: Expr_::Var(i),
		},

		Integer(i) => Expression {
			span: span!(),
			node: Expr_::Literal(i),
		},

		LParen term[a] RParen => a,
	}
}

pub fn parse<I: Iterator<Item = (Token, Span)>>(i: I) -> Result<Program, (Option<(Token, Span)>, &'static str)> {
	parse_(i)
}
