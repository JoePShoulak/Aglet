use std::sync::Mutex;
use colored::Colorize;
use crate::lexer::Span;

pub enum MessageKind {
	Error,
	Warning,
	Hint,
}

pub struct Message {
	kind: MessageKind,
	text: String,
	span: Span,
}

//Thread safety. Not necessary yet but good practice.
static DID_ERROR: Mutex<bool> = Mutex::new(false);
static MESSAGES: Mutex<Vec<Message>> = Mutex::new(vec![]);

pub fn error(text: String, span: Span) {
	let mut msgs = MESSAGES.lock().unwrap();
	let msg: Message = Message { kind: MessageKind::Error, text: text, span: span };
	msgs.push(msg);

	let mut data = DID_ERROR.lock().unwrap();
	*data = true;
}

pub fn eof_error(filename: Option<String>, full_text: &String, text: String) {
	let line_no = full_text.chars().filter(|&c| c == '\n').count() + 1;

	//Print filename, line number and column number.
	eprintln!("{}: {}", "error".red().bold(), text.bold());
	match filename {
		None => {
			eprintln!("  {} stdin:{}", "-->".bright_blue().bold(), line_no);
		},
		Some(s) => {
			eprintln!("  {} {:?}:{}", "-->".bright_blue().bold(), s, line_no);
		}
	}
	eprintln!("   {} {}", "|".bright_blue().bold(), "(EOF)".bright_blue().bold());

	let mut data = DID_ERROR.lock().unwrap();
	*data = true;
}

pub fn warn(text: String, span: Span) {
	let mut msgs = MESSAGES.lock().unwrap();
	let msg: Message = Message { kind: MessageKind::Warning, text: text, span: span };
	msgs.push(msg);
}

pub fn hint(text: String, span: Span) {
	let mut msgs = MESSAGES.lock().unwrap();
	let msg: Message = Message { kind: MessageKind::Hint, text: text, span: span };
	msgs.push(msg);
}

//Unlike the other message types, info messages print immediately.
pub fn info(text: &str) {
	eprintln!("{}: {}", "info".bold(), text);
}

pub fn errored() -> bool {
	*DID_ERROR.lock().unwrap()
}

fn print_context(filename: &Option<String>, full_text: &String, span: Span) {
	let before = &full_text[0..span.lo];
	let after = &full_text[span.hi..full_text.len()];
	let line_begin = (1 + {
		let mut ix = -1;
		for (index, c) in before.char_indices().rev() {
			if c == '\n' {
				ix = index as isize;
				break;
			}
		}
		ix
	}) as usize;
	let line_end = span.hi + {
		let mut ix = 0;
		for (index, c) in after.char_indices() {
			ix = index;
			if c == '\n' {
				break;
			}
		}
		ix
	} - 1;

	let line_no = before.chars().filter(|&c| c == '\n').count() + 1;
	let col_no = span.lo - line_begin;

	//Print filename, line number and column number.
	match filename {
		None => {
			eprintln!("  {} stdin:{}:{}", "-->".bright_blue().bold(), line_no, col_no);
		},
		Some(s) => {
			eprintln!("  {} {}:{}:{}", "-->".bright_blue().bold(), s, line_no, col_no);
		}
	}

	//Print the line in question and highlight what element is being referred to.
	eprintln!("   {}", "|".bright_blue().bold());
	eprintln!("{:<3}{} {}", format!("{}", line_no).bright_blue().bold(), "|".bright_blue().bold(), &full_text[line_begin ..= line_end]);
	eprintln!("   {} {}{}", "|".bright_blue().bold(), " ".repeat(span.lo - line_begin), "^".repeat(span.hi - span.lo).bright_blue().bold());
}

pub fn print_all(full_text: String, filename: Option<String>) {
	let msgs = MESSAGES.lock().unwrap();

	for message in msgs.iter() {
		match message.kind {
			MessageKind::Error => {
				eprintln!("{}: {}", "error".red().bold(), message.text.bold());
				print_context(&filename, &full_text, message.span);
			},

			MessageKind::Warning => {
				eprintln!("{}: {}", "warn".yellow().bold(), message.text.bold());
				print_context(&filename, &full_text, message.span);
			},

			MessageKind::Hint => {
				eprintln!("{}: {}", "hint".bright_blue().bold(), message.text);
			},
		};
	}
}