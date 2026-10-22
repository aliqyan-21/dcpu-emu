mod assembler;
// mod dcpu;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        eprintln!("usage: {} <filepath>", args[0]);
        std::process::exit(1);
    }

    let mut asmblr = assembler::assembler::Assembler::new();
    asmblr.tokenize(args[1].clone());

    // for t in asmblr.tokens.iter_mut() {
    //     t.print();
    // }

    asmblr.parse();

    // for i in asmblr.instructions {
    //     println!("{:?}", i)
    // }
    //
    // for i in asmblr.labels {
    //     println!("{:?}", i);
    // }

    asmblr.encode();

    // let mut dcpu = dcpu::dcpu::DCPU::new();
    // println!("DCPU initialized successfully!");
    // dcpu.load_rom(args[1].clone());
    // dcpu.dcpu_run();
}
