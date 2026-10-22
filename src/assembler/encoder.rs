use super::assembler::*;

// note: hex instruction will be formed like this (a << 10) | (b << 5) | op

impl Assembler {
    pub fn encode(&mut self) {
        for i in self.instructions.iter() {
            println!("{:?}", i);
            let opcode = i.opcode.code();
            let op1 = i.operand1.value();
            let op2 = match &i.operand2 {
                Some(op) => op.value(),
                None => 0x00,
            };
            let inst_code: u16 = op1 << 10 | op2 << 5 | opcode;
            println!("0x{:04x}", inst_code);
        }
    }
}
