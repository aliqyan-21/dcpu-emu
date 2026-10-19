/*
implemenation of dcpu_run
*/

use crate::dcpu::DCPU;

impl DCPU {
    pub fn decode(inst: u16) -> (u16, u16, u16) {
        let opcode = inst & 0x001F;
        let b = (inst & 0x03E0) >> 5;
        let a = (inst & 0xFC00) >> 10;
        (opcode, a, b)
    }


    pub fn dcpu_run(&mut self) {
        // self.registers[1] = u16::MAX;
        // self.registers[2] = u16::MAX;
        self.registers[1] = 0;
        self.registers[2] = 3;
        loop {
            // fetch
            let inst = self.ram[self.pc as usize];
            if inst == 0x0000 {
                println!("\nprogram finished, stopping execution...");
                break;
            }
            self.pc += 1;

            // decode
            let (opcode, a, b) = DCPU::decode(inst);
            println!("\nopcode: {opcode}, a: {a}, b: {b}");

            // execute
            match opcode {
                0x00 => {
                    todo!("special instruction")
                }
                0x01 => self.dcpu_set(a, b),
                0x02 => self.dcpu_add(a, b),
                0x03 => self.dcpu_sub(a, b),
                0x04 => self.dcpu_mul(a, b),
                0x05 => self.dcpu_mli(a, b),
                0x06 => self.dcpu_div(a, b),
                0x07 => self.dcpu_divi(a, b),
                0x08 => self.dcpu_mod(a, b),
                0x09 => self.dcpu_mdi(a, b),
                0x0a => self.dcpu_and(a, b),
                0x0b => self.dcpu_bor(a, b),
                0x0c => self.dcpu_xor(a, b),
                0x0d => self.dcpu_shr(a, b),
                0x0e => self.dcpu_asr(a, b),
                0x0f => self.dcpu_shl(a, b),
                // todo: and so on...implement more instructions
                _ => {}
            }
            self.print_state();
        }
    }
}
