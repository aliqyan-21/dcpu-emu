pub struct DCPU {
    _ram: [u16; 0x10000], // 128 KB ram
    _registers: [u16; 8], // A B C X Y Z I J
    _pc: u16,             //  program counter
    _sp: u16,             // stack pointer
    _ex: u16,             // extra/excess
    _ia: u16,             // interrupt address
}

impl DCPU {
    pub fn new() -> Self {
        Self {
            _ram: [0; 0x10000],
            _registers: [0; 8],
            _pc: 0,
            _sp: 0,
            _ex: 0,
            _ia: 0,
        }
    }
}

/*
* 0x00
* 0x01
* 0x02
*/
fn load_program(path: String) -> Vec<String> {
    let mut program: Vec<String> = Vec::new();
    let content = std::fs::read_to_string(path).expect("file could not be opened/read");
    for line in content.lines() {
        program.push(line.to_string());
    }
    return program;
}

fn main() {
    let _dcpu = DCPU::new();
    println!("DCPU initialized successfully!");
    let program: Vec<String> = load_program("file.txt".to_string());
    println!("{:?}", program);
}
