pub struct Pos {
	pub start:usize,
	pub line:usize,
	pub column:usize,
	pub end:usize,
	pub len:usize,
}

impl Pos {
	pub fn new(start:usize, line:usize, column:usize, end:usize) -> Self {
		let length = end - start;
		Self {start:start, line:line, column:column, end:end, len:length}
	}
}

pub struct Token {
	pub lexme:String,
	pub kind:TokenKind,
	pub pos:Pos,
}

impl Token {
	pub fn new(lexme:String, kind:TokenKind, pos:Pos) -> Self {
		Self {lexme, kind, pos}
	}
}

pub enum TokenKind {
	EOL,
	Space,

	OpenBrace,
	CloseBrace,
	OpenParenthesis,
	CloseParenthesis,

	Colon,
	Comma,
	Directive,
	Dash,
	Arrow,


	Comment(String),
	IdentLiteral(String),
	Number(String),
}
