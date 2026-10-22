mod assembler;
mod dcpu;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        eprintln!("usage: {} <filepath>", args[0]);
        std::process::exit(1);
    }

    // --asm file.s // take assembly file gen rom and run in dcpu
    // --rom file.rom // take rom file and run dcpu
    // --emit-rom file.s // take assembly generate rom

    // let mut asmblr = assembler::assembler::Assembler::new();
    // asmblr.tokenize(args[1].clone());

    // for t in asmblr.tokens.iter_mut() {
    //     t.print();
    // }

    // asmblr.parse();

    // for i in asmblr.instructions {
    //     println!("{:?}", i)
    // }
    //
    // for i in asmblr.labels {
    //     println!("{:?}", i);
    // }

    // asmblr.encode();

    let mut dcpu = dcpu::dcpu::DCPU::new();
    println!("DCPU initialized successfully!");
    if let Err(e) = dcpu.load_instructions(&args[1]) {
        eprintln!("[Assembler Error] {}", e);
        std::process::exit(1);
    }
    if let Err(e) = dcpu.dcpu_run() {
        eprintln!("[DCPU Error] {}", e);
        std::process::exit(1);
    }
}
