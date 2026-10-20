#[derive(Debug)]
pub enum TokenType {
    Cmd,
    Hex,
    MemPtr,
    Literal,
    Label,
}

pub struct Token {
    t_type: TokenType,
    value: String,
}

impl Token {
    pub fn new(t_type: TokenType, value: String) -> Self {
        Self { t_type, value }
    }

    pub fn print(&mut self) {
        println!("{:?}  : {}", self.t_type, self.value);
    }
}
