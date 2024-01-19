use plex::lexer;

#[derive(Debug, Clone)]
pub enum Token {
	//Ignored Tokens
	Whitespace,
	Comment,

	//Keywords
	KwdFunction,
	KwdConstant,
	KwdMutable,

	//Values
	Identifier(String),
	Integer(i64),

	//Language Structures
	LParen,
	RParen,
	LBrace,
	RBrace,
	Colon,
	Comma,
	Arrow,
	Semicolon,

	//Operators
	OperPlus,
	OperMinus,
	OperMult,
	OperDiv,
	OperMod,
	OperAssign,
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

	//Values
	"[a-zA-Z_][a-zA-Z_0-9]*" => Token::Identifier(text.to_owned()),
	"[0-9]+" => Token::Integer(text.parse().unwrap()),

	//Language Structures
	"\\(" => Token::LParen,
	"\\)" => Token::RParen,
	"\\{" => Token::LBrace,
	"\\}" => Token::RBrace,
	":" => Token::Colon,
	"," => Token::Comma,
	"->" => Token::Arrow,
	";" => Token::Semicolon,

	//Operators
	"\\+" => Token::OperPlus,
	"-" => Token::OperMinus,
	"\\*" => Token::OperMult,
	"/" => Token::OperDiv,
	"%" => Token::OperMod,
	"=" => Token::OperAssign,

	//If none of the above, raise an error!
	"." => panic!("Unexpected character \"{}\"", text),
}

pub struct Lexer<'a> {
	original: &'a str,
	remaining: &'a str,
}

impl<'a> Lexer<'a> {
	pub fn new(s: &'a str) -> Lexer<'a> {
		Lexer {
			original: s,
			remaining: s,
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

				tok => {
					return Some((tok, span));
				}
			}
		}
	}
}
