use super::assembler::*;
use std::collections::HashMap;

// note: hex instruction will be formed like this (a << 10) | (b << 5) | op

impl Assembler {
    pub fn encode(&mut self) {
        let labels = self.resolve_label_addr();
        for i in self.instructions.iter() {
            // println!("{:?}", i);
            let opcode = i.opcode.code();
            let (b_bits, b_extra) = i.operand1.value(&labels);
            let (a_bits, a_extra) = match &i.operand2 {
                Some(op) => op.value(&labels),
                None => (0x00, None),
            };
            let inst_code: u16 = (a_bits) << 10 | (b_bits) << 5 | opcode;
            self.machine_code.push(inst_code);

            if let Some(extra) = b_extra {
                self.machine_code.push(extra);
            }
            if let Some(extra) = a_extra {
                self.machine_code.push(extra);
            }
        }
    }

    fn resolve_label_addr(&self) -> HashMap<String, u16> {
        let mut inst_addrs = vec![0u16; self.instructions.len()];
        let mut curr_addr: u16 = 0;

        for (idx, inst) in self.instructions.iter().enumerate() {
            inst_addrs[idx] = curr_addr;
            let mut words = 1;

            if inst.operand1.requires_extra_word() {
                words += 1;
            }

            if let Some(op2) = &inst.operand2 {
                if op2.requires_extra_word() {
                    words += 1;
                }
            }

            curr_addr += words;
        }

        let mut resolved = HashMap::new();
        for (name, &idx) in &self.labels {
            if let Some(&addr) = inst_addrs.get(idx) {
                resolved.insert(name.clone(), addr);
            }
        }

        resolved
    }
}
