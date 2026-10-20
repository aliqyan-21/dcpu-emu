/*
implementation of all basic opcodes there is in DCPU-16
*/

use crate::dcpu::dcpu::DCPU;

impl DCPU {
    pub fn dcpu_set(&mut self, a: u16, b: u16) {
        *self.get_dst(b) = self.get_src(a);
        self.cycles += 1;
    }

    pub fn dcpu_add(&mut self, a: u16, b: u16) {
        let val_a = self.get_src(a) as u32;
        let val_b = *self.get_dst(b) as u32;
        let res = val_b + val_a;
        *self.get_dst(b) = res as u16;
        self.ex = if res > 0xFFFF { 0x0001 } else { 0x0000 };
        self.cycles += 2;
    }

    pub fn dcpu_sub(&mut self, a: u16, b: u16) {
        let val_a = self.get_src(a) as i32;
        let val_b = *self.get_dst(b) as i32;
        let res = val_b - val_a;
        *self.get_dst(b) = res as u16;
        self.ex = if res < 0 { 0xFFFF } else { 0x0000 };
        self.cycles += 2;
    }

    pub fn dcpu_mul(&mut self, a: u16, b: u16) {
        let val_a = self.get_src(a) as u32;
        let val_b = *self.get_dst(b) as u32;
        let res = val_b * val_a;
        *self.get_dst(b) = res as u16;
        self.ex = ((res >> 16) & 0xffff) as u16;
        self.cycles += 2;
    }

    pub fn dcpu_mli(&mut self, a: u16, b: u16) {
        let val_a = self.get_src(a) as i16 as i32;
        let val_b = *self.get_dst(b) as i16 as i32;
        let res = val_b * val_a;
        *self.get_dst(b) = res as u16;
        self.ex = ((res as u32 >> 16) & 0xffff) as u16;
        self.cycles += 2;
    }

    pub fn dcpu_div(&mut self, a: u16, b: u16) {
        let val_a = self.get_src(a) as u32;
        let val_b = *self.get_dst(b) as u32;
        if val_a == 0 {
            *self.get_dst(b) = 0;
            self.ex = 0;
        } else {
            let res = val_b / val_a;
            *self.get_dst(b) = res as u16;
            self.ex = ((val_b << 16) / val_a & 0xffff) as u16;
        }
        self.cycles += 3;
    }

    pub fn dcpu_dvi(&mut self, a: u16, b: u16) {
        let val_a = self.get_src(a) as i16 as i32;
        let val_b = *self.get_dst(b) as i16 as i32;
        if val_a == 0 {
            *self.get_dst(b) = 0;
            self.ex = 0;
        } else {
            let res = val_b / val_a;
            *self.get_dst(b) = res as u16;
            self.ex = (((val_b as u32) << 16) / (val_a as u32) & 0xffff) as u16;
        }
        self.cycles += 3;
    }

    pub fn dcpu_mod(&mut self, a: u16, b: u16) {
        let val_a = self.get_src(a);
        let val_b = *self.get_dst(b);
        if val_a == 0 {
            *self.get_dst(b) = 0;
        } else {
            let res = val_b % val_a;
            *self.get_dst(b) = res as u16;
        }
        self.cycles += 3;
    }

    pub fn dcpu_mdi(&mut self, a: u16, b: u16) {
        let val_a = self.get_src(a) as i16;
        let val_b = *self.get_dst(b) as i16;
        if val_a == 0 {
            *self.get_dst(b) = 0;
        } else {
            let res = val_b % val_a;
            *self.get_dst(b) = res as u16;
        }
        self.cycles += 3;
    }

    pub fn dcpu_and(&mut self, a: u16, b: u16) {
        let val_a = self.get_src(a);
        let val_b = *self.get_dst(b);
        *self.get_dst(b) = val_b & val_a;
        self.cycles += 1;
    }

    pub fn dcpu_bor(&mut self, a: u16, b: u16) {
        let val_a = self.get_src(a);
        let val_b = *self.get_dst(b);
        *self.get_dst(b) = val_b | val_a;
        self.cycles += 1;
    }

    pub fn dcpu_xor(&mut self, a: u16, b: u16) {
        let val_a = self.get_src(a);
        let val_b = *self.get_dst(b);
        *self.get_dst(b) = val_b ^ val_a;
        self.cycles += 1;
    }

    pub fn dcpu_shr(&mut self, a: u16, b: u16) {
        let val_a = (self.get_src(a) as u32).min(16);
        let val_b = *self.get_dst(b) as u32;
        *self.get_dst(b) = (val_b >> val_a) as u16;
        self.ex = ((((val_b as u32) << 16) >> val_a) & 0xffff) as u16;
        self.cycles += 1;
    }

    pub fn dcpu_asr(&mut self, a: u16, b: u16) {
        let val_a = (self.get_src(a) as u32).min(16);
        let val_b = *self.get_dst(b) as i16 as i32;
        *self.get_dst(b) = (val_b >> val_a) as u16;
        self.ex = ((((val_b as i32) << 16) >> val_a) as u32 & 0xffff) as u16;
        self.cycles += 1;
    }

    pub fn dcpu_shl(&mut self, a: u16, b: u16) {
        let val_a = (self.get_src(a) as u32).min(16);
        let val_b = *self.get_dst(b) as u32;
        *self.get_dst(b) = (val_b << val_a) as u16;
        self.ex = ((((val_b as u32) << val_a) >> 16) & 0xffff) as u16;
        self.cycles += 1;
    }

    fn skip_routine(&mut self) {
        let instr = self.ram[self.pc as usize];
        self.pc = self.pc.wrapping_add(1);
        let (_op, a, b) = DCPU::decode(instr);
        if matches!(a, 0x10..=0x17 | 0x1a | 0x1e | 0x1f) {
            self.pc = self.pc.wrapping_add(1);
            self.cycles += 1;
        }
        if matches!(b, 0x10..=0x17 | 0x1a | 0x1e | 0x1f) {
            self.pc = self.pc.wrapping_add(1);
            self.cycles += 1;
        }
    }

    pub fn dcpu_ifb(&mut self, a: u16, b: u16) {
        let val_a = self.get_src(a);
        let val_b = *self.get_dst(b);
        self.cycles += 2;
        if val_b & val_a == 0 {
            self.cycles += 1;
            self.skip_routine();
        }
    }

    pub fn dcpu_ifc(&mut self, a: u16, b: u16) {
        let val_a = self.get_src(a);
        let val_b = *self.get_dst(b);
        self.cycles += 2;
        if val_b & val_a != 0 {
            self.skip_routine();
        }
    }

    pub fn dcpu_ife(&mut self, a: u16, b: u16) {
        let val_a = self.get_src(a);
        let val_b = *self.get_dst(b);
        self.cycles += 2;
        if val_b != val_a {
            self.skip_routine();
        }
    }

    pub fn dcpu_ifn(&mut self, a: u16, b: u16) {
        let val_a = self.get_src(a);
        let val_b = *self.get_dst(b);
        self.cycles += 2;
        if val_b == val_a {
            self.skip_routine();
        }
    }

    pub fn dcpu_ifg(&mut self, a: u16, b: u16) {
        let val_a = self.get_src(a);
        let val_b = *self.get_dst(b);
        self.cycles += 2;
        if val_b <= val_a {
            self.skip_routine();
        }
    }

    pub fn dcpu_ifa(&mut self, a: u16, b: u16) {
        let val_a = self.get_src(a) as i16 as i32;
        let val_b = *self.get_dst(b) as i16 as i32;
        self.cycles += 2;
        if val_b <= val_a {
            self.skip_routine();
        }
    }

    pub fn dcpu_ifl(&mut self, a: u16, b: u16) {
        let val_a = self.get_src(a);
        let val_b = *self.get_dst(b);
        self.cycles += 2;
        if val_b >= val_a {
            self.skip_routine();
        }
    }

    pub fn dcpu_ifu(&mut self, a: u16, b: u16) {
        let val_a = self.get_src(a) as i16 as i32;
        let val_b = *self.get_dst(b) as i16 as i32;
        self.cycles += 2;
        if val_b >= val_a {
            self.skip_routine();
        }
    }

    pub fn dcpu_adx(&mut self, a: u16, b: u16) {
        let val_a = self.get_src(a) as u32;
        let val_b = *self.get_dst(b) as u32;
        let res = val_b + val_a + self.ex as u32;
        *self.get_dst(b) = res as u16;
        self.ex = if res > 0xFFFF { 0x0001 } else { 0x0000 };
        self.cycles += 3;
    }

    pub fn dcpu_sbx(&mut self, a: u16, b: u16) {
        let val_a = self.get_src(a) as i32;
        let val_b = *self.get_dst(b) as i32;
        let res = val_b - val_a + self.ex as i32;
        *self.get_dst(b) = res as u16;
        self.ex = if res < 0 { 0xFFFF } else { 0x0000 };
        self.cycles += 3;
    }

    pub fn dcpu_sti(&mut self, a: u16, b: u16) {
        self.dcpu_set(a, b);
        self.registers[6] = self.registers[6].wrapping_add(1);
        self.registers[7] = self.registers[7].wrapping_add(1);
        self.cycles += 2;
    }

    pub fn dcpu_std(&mut self, a: u16, b: u16) {
        self.dcpu_set(a, b);
        self.registers[6] = self.registers[6].wrapping_sub(1);
        self.registers[7] = self.registers[7].wrapping_sub(1);
        self.cycles += 2;
    }
}
