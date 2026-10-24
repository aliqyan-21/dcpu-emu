/* collection of all structs and enums related to the instructions */

use std::collections::HashMap;

#[rustfmt::skip]
#[derive(Debug, Clone, Copy)]
pub enum Opcode {
    Set, Add, Sub, Mul, Mli, Div, Dvi, Mod, Mdi,
    And, Bor, Xor, Shr, Asr, Shl, Ifb, Ifc, Ife,
    Ifn, Ifg, Ifa, Ifl, Ifu, Adx, Sbx, Sti, Std,
    Jsr, Int, Iag, Ias, Rfi, Iaq, Hwn, Hwq, Hwi,
}

impl Opcode {
    pub fn str(&self) -> &str {
        match self {
            Opcode::Set => "set",
            Opcode::Add => "add",
            Opcode::Sub => "sub",
            Opcode::Mul => "mul",
            Opcode::Mli => "mli",
            Opcode::Div => "div",
            Opcode::Dvi => "dvi",
            Opcode::Mod => "mod",
            Opcode::Mdi => "mdi",
            Opcode::And => "and",
            Opcode::Bor => "bor",
            Opcode::Xor => "xor",
            Opcode::Shr => "shr",
            Opcode::Asr => "asr",
            Opcode::Shl => "shl",
            Opcode::Ifb => "ifb",
            Opcode::Ifc => "ifc",
            Opcode::Ife => "ife",
            Opcode::Ifn => "ifn",
            Opcode::Ifg => "ifg",
            Opcode::Ifa => "ifa",
            Opcode::Ifl => "ifl",
            Opcode::Ifu => "ifu",
            Opcode::Adx => "adx",
            Opcode::Sbx => "sbx",
            Opcode::Sti => "sti",
            Opcode::Std => "std",
            Opcode::Jsr => "jsr",
            Opcode::Int => "int",
            Opcode::Iag => "iag",
            Opcode::Ias => "ias",
            Opcode::Rfi => "rfi",
            Opcode::Iaq => "iaq",
            Opcode::Hwn => "hwn",
            Opcode::Hwq => "hwq",
            Opcode::Hwi => "hwi",
        }
    }

    pub fn code(&self) -> u16 {
        match self {
            Opcode::Set => 0x01,
            Opcode::Add => 0x02,
            Opcode::Sub => 0x03,
            Opcode::Mul => 0x04,
            Opcode::Mli => 0x05,
            Opcode::Div => 0x06,
            Opcode::Dvi => 0x07,
            Opcode::Mod => 0x08,
            Opcode::Mdi => 0x09,
            Opcode::And => 0x0a,
            Opcode::Bor => 0x0b,
            Opcode::Xor => 0x0c,
            Opcode::Shr => 0x0d,
            Opcode::Asr => 0x0e,
            Opcode::Shl => 0x0f,
            Opcode::Ifb => 0x10,
            Opcode::Ifc => 0x11,
            Opcode::Ife => 0x12,
            Opcode::Ifn => 0x13,
            Opcode::Ifg => 0x14,
            Opcode::Ifa => 0x15,
            Opcode::Ifl => 0x16,
            Opcode::Ifu => 0x17,
            Opcode::Adx => 0x1a,
            Opcode::Sbx => 0x1b,
            Opcode::Sti => 0x1e,
            Opcode::Std => 0x1f,
            Opcode::Jsr => 0x01,
            Opcode::Int => 0x08,
            Opcode::Iag => 0x09,
            Opcode::Ias => 0x0a,
            Opcode::Rfi => 0x0b,
            Opcode::Iaq => 0x0c,
            Opcode::Hwn => 0x10,
            Opcode::Hwq => 0x11,
            Opcode::Hwi => 0x12,
        }
    }
}

#[rustfmt::skip]
#[derive(Debug, Clone)]
pub enum Register {
    A, B, C, X, Y, Z, I, J, Sp, Pc, Ex
}

impl Register {
    pub fn value(&self) -> u16 {
        match self {
            Register::A => 0x00,
            Register::B => 0x01,
            Register::C => 0x02,
            Register::X => 0x03,
            Register::Y => 0x04,
            Register::Z => 0x05,
            Register::I => 0x06,
            Register::J => 0x07,
            Register::Sp => 0x1b,
            Register::Pc => 0x1c,
            Register::Ex => 0x1d,
        }
    }
}

#[derive(Debug)]
pub enum Operand {
    Register(Register),
    Hex(u16),
    Literal(u16),
    MemPtr(u16),
    Jmp(String),
}

impl Operand {
    pub fn value(&self, labels: &HashMap<String, u16>) -> (u16, Option<u16>) {
        match self {
            Operand::Register(r) => (r.value(), None),
            Operand::Hex(h) => (0x1f, Some(*h)),
            Operand::Literal(l) => (0x1f, Some(*l)),
            Operand::MemPtr(m) => {
                if *m >= 0x08 && *m <= 0x0f {
                    (*m, None)
                } else {
                    (0x1e, Some(*m))
                }
            }
            Operand::Jmp(label_name) => {
                let addr = *labels.get(label_name).expect("Hmm...");
                (0x1f, Some(addr as u16))
            }
        }
    }

    pub fn requires_extra_word(&self) -> bool {
        match self {
            Operand::Register(_) => false,
            Operand::MemPtr(m) => {
                // Register-indirect memory pointers (0x08 to 0x0F) do not require an extra word
                !(*m >= 0x08 && *m <= 0x0f)
            }
            _ => true,
        }
    }
}

#[derive(Debug)]
pub struct Instruction {
    pub opcode: Opcode,
    pub operand1: Operand,
    pub operand2: Option<Operand>,
}

impl Instruction {
    pub fn new(opcode: Opcode, operand1: Operand, operand2: Option<Operand>) -> Self {
        Self {
            opcode,
            operand1,
            operand2,
        }
    }
}

pub enum ProgramItem {
    Instruction(Instruction),
    Data(Vec<u16>),
}
