use super::assembler::*;
use super::inst::*;
use super::token::*;

impl Assembler {
    pub fn parse(&mut self) {
        let mut inst_idx: usize = 0;
        while !self.is_end() {
            let mut token = self.advance();
            // token.print();
            match token.get_type() {
                TokenType::Label(name) => {
                    self.labels
                        .insert(name.strip_prefix(":").unwrap().to_string(), inst_idx);
                }
                TokenType::Opcode(i) => {
                    // println!("0x{:02x}", i.code());
                    let inst = self.parse_instruction(*i);
                    self.instructions.push(inst);
                    inst_idx += 1;
                }
                _ => {}
            }
        }
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

    fn parse_instruction(&mut self, opcode: Opcode) -> Instruction {
        let mut op1 = self.advance();
        let op1 = match op1.get_type() {
            TokenType::Register(r) => Operand::Register(r.clone()),
            TokenType::Hex(h) => Operand::Hex(h.clone()),
            TokenType::Literal(l) => Operand::Literal(l.clone()),
            TokenType::Jmp(j) => Operand::Jmp(j.clone()),
            TokenType::MemPtr(m) => Operand::MemPtr(m.clone()),
            _ => panic!("expected value at dst, found {:?}", op1.get_type()),
        };

        /* basic opcode takes 2nd operand */
        let op2 = if self.is_basic_opcode(opcode.str()) {
            let mut op2 = self.advance();
            let op2 = match op2.get_type() {
                TokenType::Register(r) => Operand::Register(r.clone()),
                TokenType::Hex(h) => Operand::Hex(h.clone()),
                TokenType::Literal(l) => Operand::Literal(l.clone()),
                TokenType::Jmp(j) => Operand::Jmp(j.clone()),
                TokenType::MemPtr(m) => Operand::MemPtr(m.clone()),
                _ => panic!("expected value at dst, found {:?}", op2.get_type()),
            };
            Some(op2)
        } else {
            None
        };

        Instruction::new(opcode, op1, op2)
    }
}
