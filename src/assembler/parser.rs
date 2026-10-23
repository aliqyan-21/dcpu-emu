use super::assembler::*;
use super::inst::*;
use super::token::*;

impl Assembler {
    pub fn parse(&mut self) -> Result<(), String> {
        while !self.is_end() {
            let mut token = self.advance();
            // token.print();

            match token.get_type() {
                TokenType::Label(name) => {
                    self.labels.insert(
                        name.strip_prefix(":").unwrap().to_string(),
                        self.items.len(),
                    );
                }
                TokenType::Opcode(i) => {
                    // println!("0x{:02x}", i.code());
                    let inst = self.parse_instruction(*i)?;
                    self.items.push(ProgramItem::Instruction(inst));
                }
                TokenType::DDirective(dt) => {
                    let mut chars = dt.chars().peekable();
                    let mut dataw: Vec<u16> = Vec::new();

                    while let Some(&c) = chars.peek() {
                        if c.is_whitespace() {
                            chars.next();
                        } else if c == '"' {
                            // string hai
                            chars.next();
                            while let Some(&cc) = chars.peek() {
                                if cc == '"' {
                                    chars.next();
                                    break;
                                }
                                dataw.push(cc as u16);
                                chars.next();
                            }
                        } else {
                            // koi hex value
                            let mut val = String::new();
                            while let Some(&cc) = chars.peek() {
                                if cc.is_whitespace() {
                                    break;
                                }
                                val.push(cc);
                                chars.next();
                            }

                            let val = u16::from_str_radix(&val, 16).unwrap();

                            dataw.push(val);
                        }
                    }
                    self.items.push(ProgramItem::Data(dataw));
                }
                _ => {}
            }
        }
        Ok(())
    }

    // asm structure : opcode register value

    fn is_end(&mut self) -> bool {
        self.pos >= self.tokens.len()
    }

    fn advance(&mut self) -> Token {
        let t = self.tokens[self.pos].clone();
        self.pos += 1;
        t
    }

    fn parse_instruction(&mut self, opcode: Opcode) -> Result<Instruction, String> {
        let mut op1 = self.advance();
        let op1_line = op1.line;
        let op1 = match op1.get_type() {
            TokenType::Register(r) => Operand::Register(r.clone()),
            TokenType::Hex(h) => Operand::Hex(h.clone()),
            TokenType::Literal(l) => Operand::Literal(l.clone()),
            TokenType::Jmp(j) => Operand::Jmp(j.clone()),
            TokenType::MemPtr(m) => Operand::MemPtr(m.clone()),
            _ => {
                return Err(format!(
                    "expected value/register at destination operand at line {}, found{:?}",
                    op1_line,
                    op1.get_type()
                ));
            }
        };

        /* basic opcode takes 2nd operand */
        let op2 = if self.is_basic_opcode(opcode.str()) {
            let mut op2 = self.advance();
            let op2_line = op2.line;
            let op2 = match op2.get_type() {
                TokenType::Register(r) => Operand::Register(r.clone()),
                TokenType::Hex(h) => Operand::Hex(h.clone()),
                TokenType::Literal(l) => Operand::Literal(l.clone()),
                TokenType::Jmp(j) => Operand::Jmp(j.clone()),
                TokenType::MemPtr(m) => Operand::MemPtr(m.clone()),
                _ => {
                    return Err(format!(
                        "expected value/register at source operand at line {}, found {:?}",
                        op2_line,
                        op2.get_type()
                    ));
                }
            };
            Some(op2)
        } else {
            None
        };

        Ok(Instruction::new(opcode, op1, op2))
    }
}
