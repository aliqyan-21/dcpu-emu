use crate::dcpu::DCPU;

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
                self.ram[self.sp.wrapping_add(next_word) as usize]
            }
            0x1b => self.sp,
            0x1c => self.pc,
            0x1d => self.ex,
            0x1e => {
                let next_word = self.ram[self.pc as usize];
                self.pc = self.pc.wrapping_add(1);
                self.ram[next_word as usize]
            }
            0x1f => {
                let next_word = self.ram[self.pc as usize];
                self.pc = self.pc.wrapping_add(1);
                next_word
            }
            0x20..=0x3f => (op as i32 - 0x21) as u16,
            _ => {
                panic!("no such addressing exist: {}", op);
            }
        }
    }

    pub fn get_dst(&mut self, op: u16) -> &mut u16 {
        match op {
            0x00..=0x07 => &mut self.registers[op as usize],
            0x08..=0x0f => {
                let reg_val = self.registers[(op - 0x08) as usize];
                &mut self.ram[reg_val as usize]
            }
            0x10..=0x17 => {
                let next_word = self.ram[self.pc as usize];
                self.pc = self.pc.wrapping_add(1);
                let reg_val = self.registers[(op - 0x10) as usize];
                let reg_val = reg_val.wrapping_add(next_word);
                &mut self.ram[reg_val as usize]
            }
            0x18 => {
                self.sp = self.sp.wrapping_sub(1);
                &mut self.ram[self.sp as usize]
            }
            0x19 => &mut self.ram[self.sp as usize],
            0x1a => {
                let next_word = self.ram[self.pc as usize];
                self.pc = self.pc.wrapping_add(1);
                &mut self.ram[self.sp.wrapping_add(next_word) as usize]
            }
            0x1b => &mut self.sp,
            0x1c => &mut self.pc,
            0x1d => &mut self.ex,
            0x1e => {
                let next_word = self.ram[self.pc as usize];
                self.pc = self.pc.wrapping_add(1);
                &mut self.ram[next_word as usize]
            }
            0x1f => {
                self.pc = self.pc.wrapping_add(1);
                panic!("cannot use literal as dst");
            }
            0x20..=0x3f => panic!("cannot use literal as dst"),
            _ => {
                panic!("no such addressing exist: {}", op);
            }
        }
    }
}
