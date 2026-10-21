mod assembler;
// mod dcpu;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        eprintln!("usage: {} <filepath>", args[0]);
        std::process::exit(1);
    }

    let mut asmblr = assembler::assembler::Assembler::new();
    asmblr.assemble(args[1].clone());

    for mut t in asmblr.tokens {
        t.print();
    }

    // let mut dcpu = dcpu::dcpu::DCPU::new();
    // println!("DCPU initialized successfully!");
    // dcpu.load_rom(args[1].clone());
    // dcpu.dcpu_run();
}
