/*
implemenation of dcpu_run
*/

use crate::dcpu::dcpu::DCPU;

impl DCPU {
    pub fn decode(inst: u16) -> (u16, u16, u16) {
        let opcode = inst & 0x001F;
        let b = (inst & 0x03E0) >> 5;
        let a = (inst & 0xFC00) >> 10;
        (opcode, a, b)
    }

    pub fn dcpu_run(&mut self) -> Result<(), String> {
        loop {
            // fetch
            let inst_pc = self.pc; // for graceful stopping set pc <same pc addr>

            let inst = self.ram[self.pc as usize];
            if inst == 0x0000 {
                println!("\nprogram finished, stopping execution...");
                break;
            }
            self.pc = self.pc.wrapping_add(1);

            // decode
            let (opcode, a, b) = DCPU::decode(inst);
            println!("\nopcode: {opcode}, a: {a}, b: {b}");

            // execute
            match opcode {
                0x00 => {
                    // opcode 0 hoga, b opcode hoga
                    match b {
                        0x01 => self.dcpu_jsr(a),
                        0x08 => self.dcpu_int(a),
                        0x09 => self.dcpu_iag(a)?,
                        0x0a => self.dcpu_ias(a),
                        0x0b => self.dcpu_rfi(),
                        0x0c => self.dcpu_iaq(a),
                        0x10 => self.dcpu_hwn(a),
                        0x11 => self.dcpu_hwq(a),
                        0x12 => self.dcpu_hwi(a),
                        _ => return Err(format!("opcode not supported: 0x{:04x}", b)),
                    }
                }
                0x01 => self.dcpu_set(a, b)?,
                0x02 => self.dcpu_add(a, b)?,
                0x03 => self.dcpu_sub(a, b)?,
                0x04 => self.dcpu_mul(a, b)?,
                0x05 => self.dcpu_mli(a, b)?,
                0x06 => self.dcpu_div(a, b)?,
                0x07 => self.dcpu_dvi(a, b)?,
                0x08 => self.dcpu_mod(a, b)?,
                0x09 => self.dcpu_mdi(a, b)?,
                0x0a => self.dcpu_and(a, b)?,
                0x0b => self.dcpu_bor(a, b)?,
                0x0c => self.dcpu_xor(a, b)?,
                0x0d => self.dcpu_shr(a, b)?,
                0x0e => self.dcpu_asr(a, b)?,
                0x0f => self.dcpu_shl(a, b)?,
                0x10 => self.dcpu_ifb(a, b)?,
                0x11 => self.dcpu_ifc(a, b)?,
                0x12 => self.dcpu_ife(a, b)?,
                0x13 => self.dcpu_ifn(a, b)?,
                0x14 => self.dcpu_ifg(a, b)?,
                0x15 => self.dcpu_ifa(a, b)?,
                0x16 => self.dcpu_ifl(a, b)?,
                0x17 => self.dcpu_ifu(a, b)?,
                0x1a => self.dcpu_adx(a, b)?,
                0x1b => self.dcpu_sbx(a, b)?,
                0x1e => self.dcpu_sti(a, b)?,
                0x1f => self.dcpu_std(a, b)?,
                _ => return Err(format!("opcode not suppoorted: 0x{:04x}", opcode)),
            };
            self.print_state();
            if self.pc == inst_pc {
                println!("\nprogram halted (jump to self) at 0x{:04x}", inst_pc);
                break;
            }
        }
        Ok(())
    }
}
