use super::assembler::*;
use super::token::*;
use super::utils::*;

impl Assembler {
    pub fn tokenize(&mut self, path: String) {
        let asm = read_asm(path);
        for line in asm {
            for word in line.split_whitespace() {
                let c = word.chars().nth(0).unwrap();
                let cc = if word.len() > 1 {
                    word.chars().nth(1).unwrap()
                } else {
                    '0'
                };
                if c == ':' {
                    let token = self.read_label(word);
                    self.tokens.push(token);
                } else if c.is_alphabetic() {
                    let token = self.read_identifier(word);
                    self.tokens.push(token);
                } else if c == '[' {
                    let token = self.read_memory_pointer(word);
                    self.tokens.push(token);
                } else if c == '0' && cc == 'x' {
                    if let Some(cc) = word.chars().nth(1) {
                        if cc == 'x' {
                            let token = self.read_hex_literal(word);
                            self.tokens.push(token);
                        }
                    }
                } else if c.is_numeric() {
                    let token = self.read_literal(word);
                    self.tokens.push(token);
                }
            }
        }
    }

    // 0x00fa
    fn read_hex_literal(&mut self, word: &str) -> Token {
        Token::new(TokenType::Hex(word.to_string()))
    }

    // 0 - 31
    fn read_literal(&mut self, word: &str) -> Token {
        Token::new(TokenType::Literal(word.to_string()))
    }

    // :end
    fn read_label(&mut self, word: &str) -> Token {
        self.labels
            .insert(word.strip_prefix(":").unwrap().to_string(), 100);
        Token::new(TokenType::Label(word.to_string()))
    }

    // [0xfa01]
    fn read_memory_pointer(&mut self, word: &str) -> Token {
        let word = word
            .strip_prefix("[")
            .unwrap()
            .strip_suffix("]")
            .unwrap()
            .to_string();
        Token::new(TokenType::MemPtr(word))
    }

    // a, b, loop, sum, add
    fn read_identifier(&mut self, word: &str) -> Token {
        let lower = word.to_lowercase();

        if self.labels.contains_key(word) {
            Token::new(TokenType::Jmp(word.to_string()))
        } else if self.is_register(&lower) {
            let reg = match lower.as_str() {
                "a" => Register::A,
                "b" => Register::B,
                "c" => Register::C,
                "x" => Register::X,
                "y" => Register::Y,
                "z" => Register::Z,
                "i" => Register::I,
                "j" => Register::J,
                "sp" => Register::Sp,
                "pc" => Register::Pc,
                "ex" => Register::Ex,
                _ => unreachable!(),
            };
            Token::new(TokenType::Register(reg))
        } else if self.is_opcode(&lower) {
            let op = match lower.as_str() {
                "set" => Opcode::Set,
                "add" => Opcode::Add,
                "sub" => Opcode::Sub,
                "mul" => Opcode::Mul,
                "mli" => Opcode::Mli,
                "div" => Opcode::Div,
                "dvi" => Opcode::Dvi,
                "mod" => Opcode::Mod,
                "mdi" => Opcode::Mdi,
                "and" => Opcode::And,
                "bor" => Opcode::Bor,
                "xor" => Opcode::Xor,
                "shr" => Opcode::Shr,
                "asr" => Opcode::Asr,
                "shl" => Opcode::Shl,
                "ifb" => Opcode::Ifb,
                "ifc" => Opcode::Ifc,
                "ife" => Opcode::Ife,
                "ifn" => Opcode::Ifn,
                "ifg" => Opcode::Ifg,
                "ifa" => Opcode::Ifa,
                "ifl" => Opcode::Ifl,
                "ifu" => Opcode::Ifu,
                "adx" => Opcode::Adx,
                "sbx" => Opcode::Sbx,
                "sti" => Opcode::Sti,
                "std" => Opcode::Std,
                "jsr" => Opcode::Jsr,
                "int" => Opcode::Int,
                "iag" => Opcode::Iag,
                "ias" => Opcode::Ias,
                "rfi" => Opcode::Rfi,
                "iaq" => Opcode::Iaq,
                "hwn" => Opcode::Hwn,
                "hwq" => Opcode::Hwq,
                "hwi" => Opcode::Hwi,
                _ => unreachable!(),
            };
            Token::new(TokenType::Opcode(op))
        } else {
            Token::new(TokenType::Ident(word.to_string()))
        }
    }
}
