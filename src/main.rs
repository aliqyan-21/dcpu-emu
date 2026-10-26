mod assembler;
mod common;
mod dcpu;
mod devices;

#[cfg(test)]
mod test_corpus;

fn new_dcpu(args: &common::parse_cli::Args) -> dcpu::dcpu::DCPU {
    let mut dcpu = dcpu::dcpu::DCPU::new();
    if args.clock {
        dcpu.devices.push(Box::new(devices::clock::Clock::new()));
    }
    dcpu
}

fn main() {
    let argv: Vec<String> = std::env::args().collect();
    let args = match common::parse_cli::parse_args(argv) {
        Ok(a) => a,
        Err(e) => {
            eprintln!("Error: {} try --help/-h", e);
            std::process::exit(1);
        }
    };

    if args.rom {
        let mut dcpu = new_dcpu(&args);
        println!("DCPU initialized successfully with ROM!");

        dcpu.load_rom(args.filepath);

        if let Err(e) = dcpu.dcpu_run(args.trace) {
            eprintln!("[DCPU Error] {}", e);
            std::process::exit(1);
        }
    } else if args.emit_rom {
        let mut asmblr = assembler::assembler::Assembler::new();
        if let Err(e) = asmblr.tokenize(args.filepath.clone()) {
            eprintln!("[Assembler Error] {}", e);
        }

        if let Err(e) = asmblr.parse() {
            eprintln!("[Assembler Error] {}", e);
            std::process::exit(1);
        }

        asmblr.encode();

        for inst in &asmblr.machine_code {
            println!("0x{:04x}", inst);
        }
    } else {
        let mut dcpu = new_dcpu(&args);
        println!("DCPU initialized successfully!");

        if let Err(e) = dcpu.load_instructions(&args.filepath) {
            eprintln!("[Assembler Error] {}", e);
            std::process::exit(1);
        }

        if let Err(e) = dcpu.dcpu_run(args.trace) {
            eprintln!("[DCPU Error] {}", e);
            std::process::exit(1);
        }
    }
}
