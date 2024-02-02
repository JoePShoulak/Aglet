use plex::lexer;
use crate::message;

#[derive(Debug, Clone)]
pub enum Token {
	//Ignored Tokens
	Whitespace,
	Comment,
	Unknown(String),

	//Keywords
	KwdFunction,
	KwdConstant,
	KwdMutable,
	KwdReturn,
	KwdIf,
	KwdElse,
	KwdWhile,
	KwdBreak,
	KwdContinue,
	True,
	False,

	//Keyword operators
	OperOr,
	OperAnd,
	OperXor,
	OperNot,

	//Values
	Identifier(String),
	Integer(i64),

	//Language Structures
	LParen,
	RParen,
	LBrace,
	RBrace,
	LBracket,
	RBracket,
	Colon,
	Comma,
	Arrow,
	Semicolon,
	Dot,

	//Operators
	OperPlus,
	OperMinus,
	OperMult,
	OperDiv,
	OperMod,
	OperAssign,
	OperLessThan,
	OperLessOrEqual,
	OperGreaterThan,
	OperGreaterOrEqual,
	OperEqual,
	OperNotEqual,
	OperPlusAssign,
	OperMinusAssign,
	OperMultAssign,
	OperDivAssign,
	OperModAssign,
}

lexer! {
	fn next_token(text: 'a) -> Token;

	//Ignored Tokens
	r"[ \t\r\n]" => Token::Whitespace,
	"/[*](~(.*[*]/.*))[*]/" => Token::Comment, // "C-style" comments (/* .. */) - can't contain "*/"
	r"//[^\n]*" => Token::Comment, // "C++-style" comments (// ...)

	//Keywords
	"funk" => Token::KwdFunction,
	"set" => Token::KwdConstant,
	"let" => Token::KwdMutable,
	"ret" => Token::KwdReturn,
	"if" => Token::KwdIf,
	"else" => Token::KwdElse,
	"while" => Token::KwdWhile,
	"break" => Token::KwdBreak,
	"continue" => Token::KwdContinue,
	"true" => Token::True,
	"false" => Token::False,

	//Keyword operators
	"or" => Token::OperOr,
	"and" => Token::OperAnd,
	"xor" => Token::OperXor,
	"not" => Token::OperNot,

	//Values
	"[a-zA-Z_][a-zA-Z_0-9]*" => Token::Identifier(text.to_owned()),
	"[0-9][0-9_]*" => Token::Integer(text.replace("_", "").parse().unwrap()),

	//Language Structures
	"\\(" => Token::LParen,
	"\\)" => Token::RParen,
	"\\{" => Token::LBrace,
	"\\}" => Token::RBrace,
	"\\[" => Token::LBracket,
	"\\]" => Token::RBracket,
	":" => Token::Colon,
	"," => Token::Comma,
	"->" => Token::Arrow,
	";" => Token::Semicolon,
	"\\." => Token::Dot,

	//Operators
	"\\+" => Token::OperPlus,
	"-" => Token::OperMinus,
	"\\*" => Token::OperMult,
	"/" => Token::OperDiv,
	"%" => Token::OperMod,
	"=" => Token::OperAssign,
	"<" => Token::OperLessThan,
	"<=" => Token::OperLessOrEqual,
	">" => Token::OperGreaterThan,
	">=" => Token::OperGreaterOrEqual,
	"==" => Token::OperEqual,
	"!=" => Token::OperNotEqual,
	"\\+=" => Token::OperPlusAssign,
	"-=" => Token::OperMinusAssign,
	"\\*=" => Token::OperMultAssign,
	"/=" => Token::OperDivAssign,
	"%=" => Token::OperModAssign,

	//If none of the above, raise an error!
	"." => Token::Unknown(text.to_owned()),
}

pub struct Lexer<'a> {
	original: &'a str,
	remaining: &'a str,
	context: &'a message::Context<'a>,
}

impl<'a> Lexer<'a> {
	pub fn new(context: &'a message::Context) -> Lexer<'a> {
		Lexer {
			original: context.source,
			remaining: context.source,
			context: context,
		}
	}
}

#[derive(Debug, Clone, Copy)]
pub struct Span {
	pub lo: usize,
	pub hi: usize,
}

impl<'a> Iterator for Lexer<'a> {
	type Item = (Token, Span);
	fn next(&mut self) -> Option<(Token, Span)> {
		loop {
			let (tok, span) = if let Some((tok, new_remaining)) = next_token(self.remaining) {
				let lo = self.original.len() - self.remaining.len();
				let hi = self.original.len() - new_remaining.len();
				self.remaining = new_remaining;
				(tok, Span {lo, hi})
			} else {
				return None;
			};

			match tok {
				Token::Whitespace | Token::Comment => {
					continue;
				}

				Token::Unknown(text) => {
					message::error(format!("unexpected character `{}`", text), Some(span), Some(&self.context));
					continue;
				}

				tok => {
					return Some((tok, span));
				}
			}
		}
	}
}
