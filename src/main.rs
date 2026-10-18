mod basic_opcodes;
mod dcpu;
mod dcpu_run;

fn main() {
    let mut dcpu = dcpu::DCPU::new();
    println!("DCPU initialized successfully!");
    dcpu.load_rom("file.txt".to_string());
    dcpu.dcpu_run();
}
