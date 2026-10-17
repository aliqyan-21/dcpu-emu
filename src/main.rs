pub struct DCPU {
    _ram: [u16; 0x10000], // 128 KB ram
    _registers: [u16; 8], // A B C X Y Z I J
    _pc: u16,             //  program counter
    _sp: u16,             // stack pointer
    _ex: u16,             // extra/excess
    _ia: u16,             // interrupt address
}

const START_ADDRESS: u16 = 0x0000;

impl DCPU {
    pub fn new() -> Self {
        Self {
            _ram: [0; 0x10000],
            _registers: [0; 8],
            _pc: START_ADDRESS,
            _sp: 0,
            _ex: 0,
            _ia: 0,
        }
    }

    pub fn load_rom(&mut self, program: Vec<u16>) {
        for (i, &p) in program.iter().enumerate() {
            self._ram[START_ADDRESS as usize + i] = p;
        }
        println!("Program Loaded, pc is at 0x{:04x}", self._pc)
    }
}

/*
* 0x00
* 0x01
* 0x02
*/
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

fn main() {
    let mut dcpu = DCPU::new();
    println!("DCPU initialized successfully!");
    let program: Vec<u16> = load_program("file.txt".to_string());
    dcpu.load_rom(program);
}
