use super::assembler::*;

impl Assembler {
    pub fn encode(&mut self) {
        for i in self.instructions.iter() {
            println!("{:?}", i);
        }
    }
}
