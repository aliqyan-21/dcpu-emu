use std::env;

mod addressing;
mod basic_opcodes;
mod dcpu;
mod dcpu_run;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("usage: {} <filepath>", args[0]);
        std::process::exit(1);
    }

    let mut dcpu = dcpu::DCPU::new();
    println!("DCPU initialized successfully!");
    dcpu.load_rom(args[1].clone());
    dcpu.dcpu_run();
}
