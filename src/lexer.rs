use plex::lexer;

#[derive(Debug, Clone)]
pub enum Token {
	Identifier(String),
	Integer(i64),

	Whitespace,
}

lexer! {
	fn next_token(text: 'a) -> Token;

	r"[ \t\r\n]" => Token::Whitespace,
	r"[0-9]+" => Token::Integer(text.parse().unwrap()),
	r"[a-zA-Z_][a-zA-Z_0-9]*" => Token::Identifier(text.to_owned()),
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
				Token::Whitespace => {
					continue;
				}

				tok => {
					return Some((tok, span));
				}
			}
		}
	}
}
