/*
implementation of all special opcodes there is in DCPU-16
*/

use crate::dcpu::dcpu::DCPU;

impl DCPU {
    pub fn dcpu_jsr(&mut self, a: u16) {
        let target = self.get_src(a);
        self.sp = self.sp.wrapping_sub(1);
        self.ram[self.sp as usize] = self.pc;
        self.pc = target;
        self.cycles += 3;
    }

    pub fn dcpu_int(&mut self, a: u16) -> Result<(), String> {
        let msg = self.get_src(a);
        self.cycles += 4;
        self.trigger_interrupt(msg)?;
        Ok(())
    }

    // every interrupt source use this
    pub fn trigger_interrupt(&mut self, msg: u16) -> Result<(), String> {
        if self.ia == 0 {
            return Ok(());
        }
        if self.iaq {
            if self.int_queue.len() >= 256 {
                return Err("DCPU-16 is on fire: interrupt queue overflow".to_string());
            }
            self.int_queue.push_back(msg);
        } else {
            self.enter_interrupt(msg);
        }
        Ok(())
    }

    fn enter_interrupt(&mut self, msg: u16) {
        // save
        self.iaq = true;
        self.sp = self.sp.wrapping_sub(1);
        self.ram[self.sp as usize] = self.pc;
        self.sp = self.sp.wrapping_sub(1);
        self.ram[self.sp as usize] = self.registers[0];

        // redirecting execution at IA and A = msg
        self.pc = self.ia;
        self.registers[0] = msg;
    }

    // after every instruction we will have to release one
    // queued interrupt if allowed, so for that this fn
    pub fn deliver_queued_interrupt(&mut self) {
        if self.iaq || self.ia == 0 {
            return;
        }
        if let Some(msg) = self.int_queue.pop_front() {
            self.enter_interrupt(msg);
        }
    }

    pub fn dcpu_iag(&mut self, a: u16) -> Result<(), String> {
        *self.get_dst(a)? = self.ia;
        self.cycles += 1;
        Ok(())
    }

    pub fn dcpu_ias(&mut self, a: u16) {
        self.ia = self.get_src(a);
        self.cycles += 1;
    }

    pub fn dcpu_rfi(&mut self) {
        self.iaq = false;
        self.registers[0] = self.ram[self.sp as usize];
        self.sp = self.sp.wrapping_add(1);
        self.pc = self.ram[self.sp as usize];
        self.sp = self.sp.wrapping_add(1);
        self.cycles += 3;
    }

    pub fn dcpu_iaq(&mut self, a: u16) {
        self.iaq = self.get_src(a) != 0;
        self.cycles += 2;
    }

    pub fn dcpu_hwn(&mut self, a: u16) -> Result<(), String> {
        let n = self.devices.len() as u16;
        *self.get_dst(a)? = n;
        self.cycles += 2;
        Ok(())
    }

    pub fn dcpu_hwq(&mut self, a: u16) {
        let idx = self.get_src(a) as usize;
        self.cycles += 4;

        let info = self
            .devices
            .get(idx)
            .map(|d| (d.hardware_id(), d.version(), d.manufacturer()));

        if let Some((id, ver, man)) = info {
            self.registers[0] = id as u16;
            self.registers[1] = (id >> 16) as u16;
            self.registers[2] = ver;
            self.registers[3] = man as u16;
            self.registers[4] = (man >> 16) as u16;
        }
    }

    pub fn dcpu_hwi(&mut self, a: u16) {
        let idx = self.get_src(a) as usize;
        self.cycles += 4;

        if idx < self.devices.len() {
            let mut devices = std::mem::take(&mut self.devices);
            let extra = devices[idx].interrupt(self);
            self.devices = devices;
            self.cycles += extra as u64;
        }
    }
}
