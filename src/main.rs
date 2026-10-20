mod assembler;
// mod dcpu;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        eprintln!("usage: {} <filepath>", args[0]);
        std::process::exit(1);
    }

    crate::assembler::lexer::tokenize(args[1].clone());

    // let mut dcpu = dcpu::dcpu::DCPU::new();
    // println!("DCPU initialized successfully!");
    // dcpu.load_rom(args[1].clone());
    // dcpu.dcpu_run();
}
