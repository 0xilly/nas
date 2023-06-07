
pub enum Register {
	W(String),
	WZR,
	X(String),
	XZR,
	SP,
	LR,
}

pub enum Instruction {
	Add(Register, Register, Register),
	Sub(Register, Register, Register),
	Mul(Register, Register, Register),
	Div(Register, Register, Register),
}
