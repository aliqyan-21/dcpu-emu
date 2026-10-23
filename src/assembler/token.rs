use super::inst::*;

#[derive(Debug, Clone)]
pub enum TokenType {
    Opcode(Opcode),
    Register(Register),
    Hex(u16),
    Literal(u16),
    Label(String),
    MemPtr(u16),
    Jmp(String),
    DDirective(String), // .dat
    Ident(String),
}

#[derive(Clone)]
pub struct Token {
    t_type: TokenType,
    pub line: usize,
}

impl Token {
    pub fn new(t_type: TokenType, line: usize) -> Self {
        Self { t_type, line }
    }

    pub fn get_type(&mut self) -> &TokenType {
        &self.t_type
    }

    pub fn print(&mut self) {
        println!("{} -> {:?}", self.line, self.t_type);
    }
}
