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

    fn dcpu_add(&mut self, a: u16, b: u16) {
        let val_a = self.get_src(a);
        let val_b = *self.get_dst(b);
        let (res, overflow) = val_b.overflowing_add(val_a);
        *self.get_dst(b) = res;
        self.ex = if overflow { 0x0001 } else { 0x0000 };
    }

    fn dcpu_sub(&mut self, a: u16, b: u16) {
        let val_a = self.get_src(a);
        let val_b = *self.get_dst(b);
        let (res, underflow) = val_b.overflowing_sub(val_a);
        *self.get_dst(b) = res;
        self.ex = if underflow { 0xFFFF } else { 0x0000 };
    }

    fn dcpu_mul(&mut self, a: u16, b: u16) {
        let val_a = self.get_src(a) as u32;
        let val_b = *self.get_dst(b) as u32;
        let res = val_b * val_a;
        *self.get_dst(b) = res as u16;
        self.ex = ((res >> 16) & 0xffff) as u16;
    }

    fn dcpu_mli(&mut self, a: u16, b: u16) {
        let val_a = self.get_src(a) as i16 as i32;
        let val_b = *self.get_dst(b) as i16 as i32;
        let res = val_b * val_a;
        *self.get_dst(b) = res as u16;
        self.ex = ((res as u32 >> 16) & 0xffff) as u16;
    }

    fn dcpu_div(&mut self, a: u16, b: u16) {
        let val_a = self.get_src(a) as u32;
        let val_b = *self.get_dst(b) as u32;
        if val_a == 0 {
            *self.get_dst(b) = 0;
            self.ex = 0;
        } else {
            let res = val_b / val_a;
            *self.get_dst(b) = res as u16;
            self.ex = ((val_b << 16) / val_a & 0xffff) as u16;
        }
    }

    fn dcpu_divi(&mut self, a: u16, b: u16) {
        let val_a = self.get_src(a) as i16 as i32;
        let val_b = *self.get_dst(b) as i16 as i32;
        if val_a == 0 {
            *self.get_dst(b) = 0;
            self.ex = 0;
        } else {
            let res = val_b / val_a;
            *self.get_dst(b) = res as u16;
            self.ex = (((val_b as u32) << 16) / (val_a as u32) & 0xffff) as u16;
        }
    }

    fn dcpu_mod(&mut self, a: u16, b: u16) {
        let val_a = self.get_src(a);
        let val_b = *self.get_dst(b);
        if val_a == 0 {
            *self.get_dst(b) = 0;
        } else {
            let res = val_b % val_a;
            *self.get_dst(b) = res as u16;
        }
    }

    fn dcpu_mdi(&mut self, a: u16, b: u16) {
        let val_a = self.get_src(a) as i16;
        let val_b = *self.get_dst(b) as i16;
        if val_a == 0 {
            *self.get_dst(b) = 0;
        } else {
            let res = val_b % val_a;
            *self.get_dst(b) = res as u16;
        }
    }

    fn dcpu_and(&mut self, a: u16, b: u16) {
        let val_a = self.get_src(a);
        let val_b = *self.get_dst(b);
        *self.get_dst(b) = val_b & val_a;
    }

    fn dcpu_bor(&mut self, a: u16, b: u16) {
        let val_a = self.get_src(a);
        let val_b = *self.get_dst(b);
        *self.get_dst(b) = val_b | val_a;
    }

    fn dcpu_xor(&mut self, a: u16, b: u16) {
        let val_a = self.get_src(a);
        let val_b = *self.get_dst(b);
        *self.get_dst(b) = val_b ^ val_a;
    }

    fn dcpu_shr(&mut self, a: u16, b: u16) {
        let val_a = (self.get_src(a) as u32).min(16);
        let val_b = *self.get_dst(b) as u32;
        *self.get_dst(b) = (val_b >> val_a) as u16;
        self.ex = ((((val_b as u32) << 16) >> val_a) & 0xffff) as u16;
    }

    fn dcpu_asr(&mut self, a: u16, b: u16) {
        let val_a = (self.get_src(a) as u32).min(16);
        let val_b = *self.get_dst(b) as i16 as i32;
        *self.get_dst(b) = (val_b >> val_a) as u16;
        self.ex = ((((val_b as i32) << 16) >> val_a) as u32 & 0xffff) as u16;
    }

    pub fn run(&mut self) {
        self.registers[1] = u16::MAX;
        self.registers[2] = u16::MAX;
        // self.registers[1] = 0;
        // self.registers[2] = 3;
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
                0x02 => self.dcpu_add(a, b),
                0x03 => self.dcpu_sub(a, b),
                0x04 => self.dcpu_mul(a, b),
                0x05 => self.dcpu_mli(a, b),
                0x06 => self.dcpu_div(a, b),
                0x07 => self.dcpu_divi(a, b),
                0x08 => self.dcpu_mod(a, b),
                0x09 => self.dcpu_mdi(a, b),
                0x0a => self.dcpu_and(a, b),
                0x0b => self.dcpu_bor(a, b),
                0x0c => self.dcpu_xor(a, b),
                0x0d => self.dcpu_shr(a, b),
                0x0e => self.dcpu_asr(a, b),
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
