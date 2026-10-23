pub struct Args {
    pub asm: bool,
    pub rom: bool,
    pub emit_rom: bool,
    pub filepath: String,
}

impl Args {
    pub fn new() -> Self {
        Self {
            asm: false,
            rom: false,
            emit_rom: false,
            filepath: String::new(),
        }
    }
}

pub fn usage() {
    eprintln!("Usage: dcpu_emu [--asm | --rom | --emit-rom] <filepath>");
    eprintln!("\nOptions:");
    eprintln!("--asm        take .s file as input and run dcpu");
    eprintln!("--rom        take .rom file as input and run dcpu");
    eprintln!("--emit-rom   take .s file as input and emit rom instructions");
    eprintln!("--help/-h       print this help message");
}

pub fn parse_args(argv: Vec<String>) -> Result<Args, String> {
    let mut args = Args::new();

    let mut iter = argv.into_iter().peekable();
    if iter.peek().is_some() {
        iter.next();
    }

    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "--asm" => {
                if args.rom {
                    return Err("Cannot use --asm when --rom is active.".to_string());
                }
                args.asm = true;
            }
            "--rom" => {
                if args.asm || args.emit_rom {
                    return Err("Cannot use --rom with --asm or --emit-rom.".to_string());
                }
                args.rom = true;
            }
            "--emit-rom" => {
                if args.rom {
                    return Err("Cannot use --emit-rom when --rom is active.".to_string());
                }
                args.emit_rom = true;
            }
            "--help" | "-h" => {
                usage();
                std::process::exit(0);
            }
            other => {
                if other.starts_with("--") {
                    return Err(format!("Unknown flag: {}", other));
                }
                if !args.filepath.is_empty() {
                    return Err("Multiple file paths provided.".to_string());
                }
                args.filepath = other.to_string();
            }
        }
    }

    if args.filepath.is_empty() {
        return Err("No input file path provided.".to_string());
    }

    Ok(args)
}
