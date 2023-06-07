use super::token::{Token, Pos, TokenKind};

pub(crate) struct Lex<'lex> {
	pub buffer:&'lex str,
	idx:usize,
	line:usize,
	column:usize,
	ch:u8,
	pub tokens:Vec<Token>,
}

type Kind = TokenKind;
impl<'lex> Lex <'lex> {
	pub fn new(input:&'lex str) -> Self {
		Self {
			buffer: input,
			idx: 0,
			line: 1,
			column: 1,
			ch:0,
			tokens: Vec::new(),
		}
	}

	pub fn load(&mut self) {
		while self.ch == b'\0' {
			let token = self.scan_token();
			self.tokens.push(token);
		}
	}

	fn scan_token(&mut self) -> Token {
		let _start = self.idx;
		let tok = match self.peek() {
			b'\n' => Token::new(String::from("Eol"), Kind::EOL, Pos::new(_start, self.line, self.column, self.idx)),
			b' ' | b'\t'=> Token::new(String::from("Space"), Kind::Space, Pos::new(_start, self.line, self.column, self.idx)),

			b'[' => Token::new(String::from("OpenBrace"), Kind::OpenBrace, Pos::new(_start, self.line, self.column, self.idx)),
			b']' => Token::new(String::from("CloseBrace"), Kind::CloseBrace, Pos::new(_start, self.line, self.column, self.idx)),
			b'(' => Token::new(String::from("OpenParenthesis"), Kind::OpenParenthesis, Pos::new(_start, self.line, self.column, self.idx)),
			b')' => Token::new(String::from("CloseParenthesis"), Kind::CloseParenthesis, Pos::new(_start, self.line, self.column, self.idx)),

			b'#'=> {
				let comment = self.concat_comment();
				return Token::new(String::from("Comment"), Kind::Comment(comment), Pos::new(_start, self.line, self.column, self.idx));
			},

			b':' => Token::new(String::from("Colon"), Kind::Colon, Pos::new(_start, self.line, self.column, self.idx)),
			b',' => Token::new(String::from("Comma"), Kind::Comma, Pos::new(_start, self.line, self.column, self.idx)),
			b'@' => Token::new(String::from("Directive"), Kind::Directive, Pos::new(_start, self.line, self.column, self.idx)),
			b'-' => {
				if self.check_n(1, b'>') {
					self.advance();
					return Token::new(String::from("Arrow"), Kind::Arrow, Pos::new(_start, self.line, self.column, self.idx));
				}
				return Token::new(String::from("Dash"), Kind::Dash, Pos::new(_start, self.line, self.column, self.idx));
			}
			b'a' ..= b'z' | b'A' ..= b'Z' => self.consume_ident(),
			b'0' ..= b'9' => self.consume_number(),

			_ => Token::new(String::from("Invalid"), Kind::EOL, Pos::new(999999999, 99999999, 99999999, 99999999))
		};
		return tok
	}

	fn consume_number(&mut self) -> Token {
			let start = self.idx;

			loop {
				match self.ch {
					b'0' ..= b'9' => {
						self.advance();
					},
					_ => {
						break;
					},
				}
			}

			let literal = &self.buffer[start..self.idx];
			return Token::new(String::from("NumberLiteral"), Kind::Number(literal.to_string()), Pos::new(start, self.line, self.column, self.idx));

	}

	fn consume_ident(&mut self) -> Token {
			let start = self.idx;

			loop {
				match self.ch {
					b'a'..=b'z' | b'A'..=b'Z' | b'_' => {
						self.advance();
					},
					_ => {
						break;
					},
				}
			}

			let literal = &self.buffer[start..self.idx];
			return Token::new(String::from("IdentLiteral"), Kind::IdentLiteral(literal.to_string()), Pos::new(start, self.line, self.column, self.idx));
	}

	fn concat_comment(&mut self) -> String {
		let start = self.idx;
		while self.ch != b'\n' {
			self.advance();
		}
		self.idx -= 1; //Note(anita): have to reverse the parser by 1 to keep the newline
		let comment = &self.buffer[start..self.idx];

		return comment.to_string();
	}


	fn peek(&mut self) -> u8 {
		return self.peek_n(0);
	}

	fn peek_n(&mut self, n: usize) -> u8 {
		return self.buffer.as_bytes()[self.idx + n];
	}

	fn check_n(&mut self, n: usize, c: u8) -> bool {
		return self.peek_n(n) == c;
	}

	fn advance(&mut self) {
		self.idx += 1;
		self.column += 1;
		self.ch = self.peek();
	}


}

