/*
dcpu struct and important initializing and prining functions
*/

pub struct DCPU {
    pub ram: [u16; 0x10000], // 128 KB ram
    pub registers: [u16; 8], // A B C X Y Z I J
    pub pc: u16,             //  program counter
    pub sp: u16,             // stack pointer
    pub ex: u16,             // extra/excess
    pub ia: u16,             // interrupt address
    pub iaq: bool,           // interrupt queue
}

const START_ADDRESS: u16 = 0x0000;

impl DCPU {
    pub fn new() -> Self {
        Self {
            ram: [0; 0x10000],
            registers: [0; 8],
            pc: START_ADDRESS,
            sp: 0,
            ex: 0,
            ia: 0,
        }
    }

    pub fn print_state(&mut self) {
        println!(
            "A: 0x{:04x}  B: 0x{:04x}  C: 0x{:04x}  X: 0x{:04x}",
            self.registers[0], self.registers[1], self.registers[2], self.registers[3],
        );
        println!(
            "Y: 0x{:04x}  Z: 0x{:04x}  I: 0x{:04x}  J: 0x{:04x}",
            self.registers[4], self.registers[5], self.registers[6], self.registers[7]
        );
        println!(
            "PC: 0x{:04x}  SP: 0x{:04x}  EX: 0x{:04x}  IA: 0x{:04x}",
            self.pc, self.sp, self.ex, self.ia
        );
    }

    fn load_program(path: String) -> Vec<u16> {
        let mut program: Vec<u16> = Vec::new();
        let content = std::fs::read_to_string(path).expect("file could not be opened/read");
        for line in content.lines() {
            let code = line.trim().to_lowercase();
            let code = u16::from_str_radix(code.strip_prefix("0x").unwrap_or(&code), 16)
                .expect("street fighter");
            program.push(code);
        }
        return program;
    }

    pub fn load_rom(&mut self, path: String) {
        let program: Vec<u16> = DCPU::load_program(path);
        for (i, &p) in program.iter().enumerate() {
            self.ram[START_ADDRESS as usize + i] = p;
        }
        println!("Program Loaded, pc is at 0x{:04x}", self.pc)
    }
}
