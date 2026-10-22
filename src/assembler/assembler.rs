use super::inst::*;
use super::token::*;
use std::collections::HashMap;

pub struct Assembler {
    pub tokens: Vec<Token>,
    pub instructions: Vec<Instruction>,
    pub labels: HashMap<String, usize>, // [label name : inst index]
    pub pos: usize,
}

impl Assembler {
    pub fn new() -> Self {
        Self {
            tokens: Vec::new(),
            instructions: Vec::new(),
            labels: HashMap::new(),
            pos: 0,
        }
    }

    const BOPCODES: &[&str]  /* basic opcodes */ = &[
        "set", "add", "sub", "mul", "mli", "div", "dvi", "mod", "mdi", "and", "bor", "xor", "shr",
        "asr", "shl", "ifb", "ifc", "ife", "ifn", "ifg", "ifa", "ifl", "ifu", "adx", "sbx", "sti",
        "std",
    ];

    const SOPCODES: &[&str]  /* special opcodes */ = &[
        "jsr", "int", "iag", "ias", "rfi", "iaq", "hwn", "hwq", "hwi",
    ];

    const REGISTERS: &[&str] = &["a", "b", "c", "x", "y", "z", "i", "j", "sp", "pc", "ex"];

    pub fn is_opcode(&mut self, word: &str) -> bool {
        Self::BOPCODES.contains(&word) || Self::SOPCODES.contains(&word)
    }

    pub fn is_basic_opcode(&mut self, word: &str) -> bool {
        Self::BOPCODES.contains(&word)
    }

    pub fn is_register(&mut self, word: &str) -> bool {
        Self::REGISTERS.contains(&word)
    }
}
