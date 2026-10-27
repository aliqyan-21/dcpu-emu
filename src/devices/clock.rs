use super::super::dcpu::dcpu::DCPU;
use super::device::*;

#[derive(Default)]
pub struct Clock {
    start: u64,   // cycle count at A = 0
    period: u64,  // cycles per tick (0 will mean clock is off)
    int_msg: u16, // 0 means interrupts are disabled
    fired: u64,   // ticks as interrupts
}

impl Clock {
    pub fn new() -> Self {
        Self::default()
    }

    fn ticks_at(&self, now: u64) -> u64 {
        if self.period == 0 {
            0
        } else {
            now.saturating_sub(self.start) / self.period
        }
    }
}

impl Device for Clock {
    fn hardware_id(&self) -> u32 {
        0x12d0_b402
    }
    fn version(&self) -> u16 {
        1
    }
    fn manufacturer(&self) -> u32 {
        0
    }

    fn interrupt(&mut self, dcpu: &mut DCPU) -> u32 {
        let now = dcpu.cycles;
        match dcpu.registers[0] {
            0 => {
                let b = dcpu.registers[1] as u64;
                self.start = now;
                self.period = if b == 0 { 0 } else { CPU_HZ * b / 60 };
                self.fired = 0;
            }

            1 => dcpu.registers[2] = self.ticks_at(now) as u16,

            2 => {
                self.int_msg = dcpu.registers[1];
                self.fired = self.ticks_at(now);
            }

            _ => {}
        }
        0
    }

    fn tick(&mut self, now: u64) -> Option<u64> {
        if self.int_msg == 0 {
            return None;
        }
        if self.ticks_at(now) > self.fired {
            self.fired += 1;
            Some(self.int_msg.into())
        } else {
            None
        }
    }
}
