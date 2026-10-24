/* implementation of the encoder which takes parsed instruction and turns them in machine code */

use super::assembler::*;
use super::inst::*;
use std::collections::HashMap;

// note: hex instruction will be formed like this (a << 10) | (b << 5) | op

impl Assembler {
    pub fn encode(&mut self) {
        let labels = self.resolve_label_addr();
        for item in self.items.iter() {
            match item {
                ProgramItem::Instruction(inst) => {
                    // println!("{:?}", i);
                    let opcode = inst.opcode.code();
                    let (first_bits, first_extra) = inst.operand1.value(&labels);
                    let (inst_code, a_extra, b_extra) = match &inst.operand2 {
                        Some(op2) => {
                            let (a_bits, a_extra) = op2.value(&labels);
                            (
                                (a_bits << 10) | (first_bits << 5) | opcode,
                                a_extra,
                                first_extra,
                            )
                        }
                        None => ((first_bits << 10) | (opcode << 5), first_extra, None),
                    };
                    self.machine_code.push(inst_code);

                    if let Some(extra) = a_extra {
                        self.machine_code.push(extra);
                    }
                    if let Some(extra) = b_extra {
                        self.machine_code.push(extra);
                    }
                }
                ProgramItem::Data(dt) => {
                    for word in dt {
                        self.machine_code.push(*word);
                    }
                }
            }
        }
    }

    fn resolve_label_addr(&mut self) -> HashMap<String, u16> {
        let mut item_addrs = vec![0u16; self.items.len()];
        let mut curr_addr: u16 = 0;

        for (idx, item) in self.items.iter().enumerate() {
            item_addrs[idx] = curr_addr;
            match item {
                ProgramItem::Instruction(inst) => {
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
                ProgramItem::Data(dt) => {
                    curr_addr += dt.len() as u16;
                }
            }
        }

        let mut resolved = HashMap::new();
        for (name, &idx) in &self.labels {
            if let Some(&addr) = item_addrs.get(idx) {
                resolved.insert(name.clone(), addr);
            } else {
                /* this for situation when label is there
                 but no instructions defined after that
                like:
                    :start
                     set a 0x0001
                     set pc end     ; jump over the data section

                    :message
                      .dat "Hello"

                    :end
                */
                resolved.insert(name.clone(), curr_addr);
            }
        }

        resolved
    }
}
