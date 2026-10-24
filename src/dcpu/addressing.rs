pub use crate::dcpu::dcpu::DCPU;

impl DCPU {
    pub fn get_src(&mut self, op: u16) -> u16 {
        match op {
            0x00..=0x07 => self.registers[op as usize],
            0x08..=0x0f => {
                let reg_val = self.registers[(op - 0x08) as usize];
                self.ram[reg_val as usize]
            }
            0x10..=0x17 => {
                let next_word = self.ram[self.pc as usize];
                self.pc = self.pc.wrapping_add(1);
                let reg_val = self.registers[(op - 0x10) as usize];
                let reg_val = reg_val.wrapping_add(next_word);
                self.cycles += 1;
                self.ram[reg_val as usize]
            }
            0x18 => {
                let val = self.ram[self.sp as usize];
                self.sp = self.sp.wrapping_add(1);
                val
            }
            0x19 => self.ram[self.sp as usize],
            0x1a => {
                let next_word = self.ram[self.pc as usize];
                self.pc = self.pc.wrapping_add(1);
                self.cycles += 1;
                self.ram[self.sp.wrapping_add(next_word) as usize]
            }
            0x1b => self.sp,
            0x1c => self.pc,
            0x1d => self.ex,
            0x1e => {
                let next_word = self.ram[self.pc as usize];
                self.pc = self.pc.wrapping_add(1);
                self.cycles += 1;
                self.ram[next_word as usize]
            }
            0x1f => {
                let next_word = self.ram[self.pc as usize];
                self.pc = self.pc.wrapping_add(1);
                self.cycles += 1;
                next_word
            }
            0x20..=0x3f => (op as i32 - 0x20) as u16,
            _ => unreachable!(),
        }
    }

    pub fn get_dst(&mut self, op: u16) -> Result<&mut u16, String> {
        match op {
            0x00..=0x07 => Ok(&mut self.registers[op as usize]),
            0x08..=0x0f => {
                let reg_val = self.registers[(op - 0x08) as usize];
                Ok(&mut self.ram[reg_val as usize])
            }
            0x10..=0x17 => {
                let next_word = self.ram[self.pc as usize];
                self.pc = self.pc.wrapping_add(1);
                let reg_val = self.registers[(op - 0x10) as usize];
                let reg_val = reg_val.wrapping_add(next_word);
                self.cycles += 1;
                Ok(&mut self.ram[reg_val as usize])
            }
            0x18 => {
                self.sp = self.sp.wrapping_sub(1);
                Ok(&mut self.ram[self.sp as usize])
            }
            0x19 => Ok(&mut self.ram[self.sp as usize]),
            0x1a => {
                let next_word = self.ram[self.pc as usize];
                self.pc = self.pc.wrapping_add(1);
                self.cycles += 1;
                Ok(&mut self.ram[self.sp.wrapping_add(next_word) as usize])
            }
            0x1b => Ok(&mut self.sp),
            0x1c => Ok(&mut self.pc),
            0x1d => Ok(&mut self.ex),
            0x1e => {
                let next_word = self.ram[self.pc as usize];
                self.pc = self.pc.wrapping_add(1);
                self.cycles += 1;
                Ok(&mut self.ram[next_word as usize])
            }
            0x1f => {
                let addr = self.pc as usize;
                self.pc = self.pc.wrapping_add(1);
                self.cycles += 1;
                Ok(&mut self.ram[addr])
            }
            0x20..=0x3f => Err(format!("cannot use literal as dst")),
            _ => unreachable!(),
        }
    }
}
