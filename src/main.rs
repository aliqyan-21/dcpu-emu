pub struct DCPU {
    ram: [u16; 0x10000], // 128 KB ram
    registers: [u16; 8], // A B C X Y Z I J
    pc: u16,             //  program counter
    sp: u16,             // stack pointer
    ex: u16,             // extra/excess
    ia: u16,             // interrupt address
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

    fn print_state(&mut self) {
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

    fn decode(inst: u16) -> (u16, u16, u16) {
        let opcode = inst & 0x001F;
        let b = (inst & 0x03E0) >> 5;
        let a = (inst & 0xFC00) >> 10;
        (opcode, a, b)
    }

    fn get_src(&mut self, op: u16) -> u16 {
        match op {
            0x00..=0x07 => self.registers[op as usize],
            _ => {
                todo!("exhaust")
            }
        }
    }

    fn get_dst(&mut self, op: u16) -> &mut u16 {
        match op {
            0x00..=0x07 => return &mut self.registers[op as usize],
            _ => {
                todo!("exhaust")
            }
        }
    }

    fn add(&mut self, a: u16, b: u16) {
        let val_a = self.get_src(a);
        let val_b = *self.get_dst(b);
        let (res, overflow) = val_b.overflowing_add(val_a);
        *self.get_dst(b) = res;
        self.ex = if overflow { 0x0001 } else { 0x0000 };
    }

    fn sub(&mut self, a: u16, b: u16) {
        let val_a = self.get_src(a);
        let val_b = *self.get_dst(b);
        let (res, underflow) = val_b.overflowing_sub(val_a);
        *self.get_dst(b) = res;
        self.ex = if underflow { 0xFFFF } else { 0x0000 };
    }

    fn mul(&mut self, a: u16, b: u16) {
        let val_a = self.get_src(a) as u32;
        let val_b = *self.get_dst(b) as u32;
        let res = val_b * val_a;
        *self.get_dst(b) = res as u16;
        self.ex = ((res >> 16) & 0xffff) as u16;
    }

    pub fn run(&mut self) {
        self.registers[1] = 0; // B = 0
        self.registers[2] = 3; // C = 2
        loop {
            // fetch
            let inst = self.ram[self.pc as usize];
            if inst == 0x0000 {
                println!("\nprogram finished, stopping execution...");
                break;
            }
            self.pc += 1;

            // decode
            let (opcode, a, b) = DCPU::decode(inst);
            println!("\nopcode: {opcode}, a: {a}, b: {b}");

            // execute
            match opcode {
                0x00 => {
                    todo!("special instruction")
                }
                0x01 => *self.get_dst(b) = self.get_src(a),
                0x02 => self.add(a, b),
                0x03 => self.sub(a, b),
                0x04 => self.mul(a, b),
                // todo: and so on...implement more instructions
                _ => {}
            }
            self.print_state();
        }
    }
}

/*
* 0x00
* 0x01
* 0x02
*/

fn main() {
    let mut dcpu = DCPU::new();
    println!("DCPU initialized successfully!");
    dcpu.load_rom("file.txt".to_string());
    dcpu.run();
}
