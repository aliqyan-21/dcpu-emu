#[rustfmt::skip]
#[derive(Debug)]
pub enum Opcode {
    Set, Add, Sub, Mul, Mli, Div, Dvi, Mod, Mdi,
    And, Bor, Xor, Shr, Asr, Shl, Ifb, Ifc, Ife,
    Ifn, Ifg, Ifa, Ifl, Ifu, Adx, Sbx, Sti, Std,
    Jsr, Int, Iag, Ias, Rfi, Iaq, Hwn, Hwq, Hwi,
}

#[rustfmt::skip]
#[derive(Debug)]
pub enum Register {
    A, B, C, X, Y, Z, I, J, Sp, Pc, Ex,
}

#[derive(Debug)]
pub enum TokenType {
    Opcode(Opcode),
    Register(Register),
    Hex(u16),
    Literal(u16),
    Label(String),
    MemPtr(u16),
    Jmp(String),
    //todo: .dat and string
    Ident(String),
}

pub struct Token {
    t_type: TokenType,
    line: usize,
}

impl Token {
    pub fn new(t_type: TokenType, line: usize) -> Self {
        Self { t_type, line }
    }

    pub fn print(&mut self) {
        println!("{} -> {:?}", self.line, self.t_type);
    }
}
