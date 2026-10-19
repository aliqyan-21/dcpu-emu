/*
implementation of all basic opcodes there is in DCPU-16
*/

use crate::dcpu::DCPU;

impl DCPU {
    pub fn dcpu_jsr(&mut self, a: u16) {
        let target = self.get_src(a);
        self.sp = self.sp.wrapping_sub(1);
        self.ram[self.sp as usize] = self.pc;
        self.pc = target;
    }

    pub fn dcpu_int(&mut self, a: u16) {
        if self.ia == 0 {
            return;
        }
        let msg = self.get_src(a);
        self.iaq = true;

        // save
        self.sp = self.sp.wrapping_sub(1);
        self.ram[self.sp as usize] = self.pc;
        self.sp = self.sp.wrapping_sub(1);
        self.ram[self.sp as usize] = self.registers[0];

        // redirecting execution at IA and A = msg
        self.pc = self.ia;
        self.registers[0] = msg;
    }

    pub fn dcpu_iag(&mut self, a: u16) {
        *self.get_dst(a) = self.ia;
    }

    pub fn dcpu_ias(&mut self, a: u16) {
        self.ia = self.get_src(a);
    }

    pub fn dcpu_rfi(&mut self) {
        self.iaq = false;
        self.registers[0] = self.ram[self.sp as usize];
        self.sp = self.sp.wrapping_add(1);
        self.pc = self.ram[self.sp as usize];
        self.sp = self.sp.wrapping_add(1);
    }

    pub fn dcpu_iaq(&mut self, a: u16) {
        self.iaq = self.get_src(a) != 0;
    }
}
