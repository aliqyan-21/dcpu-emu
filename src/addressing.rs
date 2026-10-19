use crate::dcpu::DCPU;

impl DCPU {
    pub fn get_src(&mut self, op: u16) -> u16 {
        match op {
            0x00..=0x07 => self.registers[op as usize],
            _ => {
                todo!("exhaust")
            }
        }
    }

    pub fn get_dst(&mut self, op: u16) -> &mut u16 {
        match op {
            0x00..=0x07 => &mut self.registers[op as usize],
            _ => {
                todo!("exhaust")
            }
        }
    }
}
