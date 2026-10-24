/* lexer */

use super::assembler::*;
use super::inst::*;
use super::token::*;
use super::utils::*;

impl Assembler {
    pub fn tokenize(&mut self, path: String) -> Result<(), String> {
        let asm = read_asm(path);
        self.collect_labels(&asm);
        self.tokenize_lines(&asm)?;
        Ok(())
    }

    fn tokenize_lines(&mut self, asm: &Vec<String>) -> Result<(), String> {
        for (n, line) in asm.iter().enumerate() {
            let line = normalize(line);
            if line.starts_with(".dat") {
                let token = self.read_data_directive(&line, n + 1);
                self.tokens.push(token);
                continue;
            }
            // println!("{line}");
            let line = line.to_lowercase();
            for word in line.split_whitespace() {
                let c = word.chars().nth(0).unwrap();
                let cc = if word.len() > 1 {
                    word.chars().nth(1).unwrap()
                } else {
                    '0'
                };
                if c == ':' {
                    let token = self.read_label(word, n + 1);
                    self.tokens.push(token);
                } else if c.is_alphabetic() {
                    let token = self.read_identifier(word, n + 1);
                    self.tokens.push(token);
                } else if c == '[' {
                    let token = self.read_memory_pointer(word, n + 1);
                    self.tokens.push(token);
                } else if c == '0' && cc == 'x' {
                    if let Some(cc) = word.chars().nth(1) {
                        if cc == 'x' {
                            let token = self.read_hex_literal(word, n + 1);
                            self.tokens.push(token);
                        }
                    }
                } else if c.is_numeric() {
                    let token = self.read_literal(word, n + 1);
                    self.tokens.push(token);
                } else {
                    return Err(format!("Unknown symbol: '{}' at line  {}", word, n + 1));
                }
            }
        }
        Ok(())
    }

    fn collect_labels(&mut self, asm: &Vec<String>) {
        for line in asm.iter() {
            for word in line.split_whitespace() {
                if word.starts_with(":") {
                    self.labels
                        .insert(word.strip_prefix(":").unwrap().to_string(), 100);
                }
            }
        }
    }

    // 0x00fa
    fn read_hex_literal(&mut self, word: &str, n: usize) -> Token {
        let word = u16::from_str_radix(word.strip_prefix("0x").unwrap_or(&word), 16)
            .expect("street fighter in lexer");
        Token::new(TokenType::Hex(word), n)
    }

    // 0 - 31
    fn read_literal(&mut self, word: &str, n: usize) -> Token {
        let word = u16::from_str_radix(word.strip_prefix("0x").unwrap_or(&word), 10)
            .expect("street fighter in lexer");
        Token::new(TokenType::Literal(word), n)
    }

    fn read_data_directive(&mut self, line: &str, n: usize) -> Token {
        let line = line.strip_prefix(".dat").unwrap().trim();
        Token::new(TokenType::DDirective(line.to_string()), n)
    }

    // :end
    fn read_label(&mut self, word: &str, n: usize) -> Token {
        self.labels
            .insert(word.strip_prefix(":").unwrap().to_string(), 0x000);
        Token::new(TokenType::Label(word.to_string()), n)
    }

    // [0xfa01] / [a] / [I]
    fn read_memory_pointer(&mut self, word: &str, n: usize) -> Token {
        let word = word
            .strip_prefix("[")
            .unwrap()
            .strip_suffix("]")
            .unwrap()
            .trim();

        // [a] - [j]
        if self.is_register(word) {
            let reg = Register::str(word);
            return Token::new(TokenType::MemPtr(0x08 + reg.value(), None), n);
        }

        // [0x1000]
        if let Some(val) = Self::parse_number(&word) {
            return Token::new(TokenType::MemPtr(0x1e, Some(val)), n);
        }

        // [0x2000+i] or [sp+4]
        if word.contains('+') {
            let parts: Vec<&str> = word.split('+').map(|s| s.trim()).collect();
            if parts.len() == 2 {
                let mut reg_code = None;
                let mut is_sp = false;
                let mut offset = None;

                for p in parts {
                    if self.is_register(p) {
                        if p == "sp" {
                            is_sp = true;
                        } else {
                            reg_code = Some(Register::str(p).value());
                        }
                    } else if let Some(v) = Self::parse_number(p) {
                        offset = Some(v)
                    }
                }

                if let Some(val) = offset {
                    if is_sp {
                        return Token::new(TokenType::MemPtr(0x1a, Some(val)), n);
                    } else if let Some(r) = reg_code {
                        return Token::new(TokenType::MemPtr(0x10 + r, Some(val)), n);
                    }
                }
            }
        }
        panic!("Unknown memory pointer format: [{}] at line {}", word, n);
    }

    fn parse_number(s: &str) -> Option<u16> {
        if s.starts_with("0x") {
            u16::from_str_radix(s.strip_prefix("0x").unwrap(), 16).ok()
        } else {
            u16::from_str_radix(s, 10).ok()
        }
    }

    // a, b, loop, sum, add
    fn read_identifier(&mut self, word: &str, n: usize) -> Token {
        if self.is_stackop(word) {
            let code = if word == "peek" { 0x19 } else { 0x18 };
            return Token::new(TokenType::MemPtr(code, None), n);
        }
        if self.labels.contains_key(word) {
            Token::new(TokenType::Jmp(word.to_string()), n)
        } else if self.is_register(&word) {
            let reg = match word {
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
            Token::new(TokenType::Register(reg), n)
        } else if self.is_opcode(&word) {
            let op = match word {
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
            Token::new(TokenType::Opcode(op), n)
        } else {
            Token::new(TokenType::Ident(word.to_string()), n)
        }
    }
}
