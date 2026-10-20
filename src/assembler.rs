use std::fs;

enum _Command {
    /* basic opcodes (takes dst and src, in that order) */
    SET,
    ADD,
    SUB,
    MUL,
    MLI,
    DIV,
    DVI,
    MOD,
    MDI,
    AND,
    BOR,
    XOR,
    SHR,
    ASR,
    SHL,
    IFB,
    IFC,
    IFE,
    IFN,
    IFG,
    IFA,
    IFL,
    IFU,
    ADX,
    SBX,
    STI,
    STD,

    /* special opcodes (these takes only one arg) */
    JSR,
    INT,
    IAG,
    IAS,
    RFI,
    IAQ,
    HWN,
    HWQ,
    HWI,
}

enum _TokenType {
    CMD,
    HEX,
    LBRACKET,
    RBRACKET,
    LITERAL,
    LABEL,
}

fn is_comment(line: &str) -> bool {
    line.starts_with(';')
}

fn normalize(line: &str) -> String {
    let mut line = line.trim().to_string();

    if is_comment(&line) {
        return String::new();
    };

    if let Some(s) = line.find(';') {
        line.truncate(s);
    }

    line.trim().to_string()
}

pub fn read_asm(path: String) {
    let asm = fs::read_to_string(path).expect("file could not be opened: {path}");
    for (i, line) in asm.lines().enumerate() {
        let line = normalize(line);
        if !line.is_empty() {
            println!("{i} : {line}");
        }
    }
}
