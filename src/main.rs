mod assembler;
mod common;
mod dcpu;

fn main() {
    let argv: Vec<String> = std::env::args().collect();
    let args = match common::parse_cli::parse_args(argv) {
        Ok(a) => a,
        Err(e) => {
            eprintln!("Error: {}", e);
            common::parse_cli::usage();
            std::process::exit(1);
        }
    };

    if args.rom {
        let mut dcpu = dcpu::dcpu::DCPU::new();
        println!("DCPU initialized successfully with ROM!");

        dcpu.load_rom(args.filepath);

        if let Err(e) = dcpu.dcpu_run() {
            eprintln!("[DCPU Error] {}", e);
            std::process::exit(1);
        }
    } else if args.emit_rom {
        let mut asmblr = assembler::assembler::Assembler::new();
        _ = asmblr.tokenize(args.filepath.clone()); // todo: not return the tokens

        if let Err(e) = asmblr.parse() {
            eprintln!("[Assembler Error] {}", e);
            std::process::exit(1);
        }

        asmblr.encode();

        for inst in &asmblr.machine_code {
            println!("0x{:04x}", inst);
        }
    } else {
        let mut dcpu = dcpu::dcpu::DCPU::new();
        println!("DCPU initialized successfully!");

        if let Err(e) = dcpu.load_instructions(&args.filepath) {
            eprintln!("[Assembler Error] {}", e);
            std::process::exit(1);
        }

        if let Err(e) = dcpu.dcpu_run() {
            eprintln!("[DCPU Error] {}", e);
            std::process::exit(1);
        }
    }
}
