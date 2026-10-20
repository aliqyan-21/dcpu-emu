use super::token::*;
use std::collections::HashMap;

pub struct Assembler {
    pub tokens: Vec<Token>,
    pub labels: HashMap<String, u16>, // [label name : mem addr]
}

impl Assembler {
    pub fn new() -> Self {
        Self {
            tokens: Vec::new(),
            labels: HashMap::new(),
        }
    }

    const KEYWORDS: &[&str] = &[
        "set", "add", "sub", "mul", "mli", "div", "dvi", "mod", "mdi", "and", "bor", "xor", "shr",
        "asr", "shl", "ifb", "ifc", "ife", "ifn", "ifg", "ifa", "ifl", "ifu", "adx", "sbx", "sti",
        "std", "jsr", "int", "iag", "ias", "rfi", "iaq", "hwn", "hwq", "hwi",
    ];

    const REGISTERS: &[&str] = &["a", "b", "c", "x", "y", "z", "i", "j", "sp", "pc", "ex"];

    pub fn is_opcode(&mut self, word: &str) -> bool {
        Self::KEYWORDS.contains(&word)
    }

    pub fn is_register(&mut self, word: &str) -> bool {
        Self::REGISTERS.contains(&word)
    }
}
