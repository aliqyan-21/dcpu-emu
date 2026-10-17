pub struct DCPU {
    ram: [u16; 0x10000], // 128 KB ram
    registers: [u16; 8], // A B C X Y Z I J
    pc: u16,             //  program counter
    sp: u16,             // stack pointer
    ex: u16,             // extra/excess
    ia: u16,             // interrupt address
}

fn main() {
    println!("Hello, world!");
}
