pub use std::fs;

fn normalize(line: &str) -> String {
    let mut line = line.trim().to_string();

    if line.starts_with(';') {
        return String::new();
    };

    if let Some(s) = line.find(';') {
        line.truncate(s);
    }

    line.trim().to_string().to_lowercase()
}

fn read_asm(path: String) -> Vec<String> {
    let mut asm = Vec::new();
    let asmc = fs::read_to_string(path).expect("file could not be opened: {path}");
    for line in asmc.lines() {
        let line = normalize(line);
        if !line.is_empty() {
            asm.push(line);
        }
    }
    asm
}

pub fn tokenize(path: String) {
    let asm = read_asm(path);
    for line in asm {
        for word in line.split_whitespace() {
            if let Some(c) = word.chars().nth(0) {
                if c == ':' {
                    println!("found label: {word}");
                } else if c.is_alphabetic() {
                    println!("found identifier: {word}");
                } else if c == '[' {
                    println!("found memory pointer: {word}");
                } else if c == '0' {
                    if let Some(cc) = word.chars().nth(1) {
                        if cc == 'x' {
                            println!("found literal: {word}");
                        }
                    }
                } else if c.is_numeric() {
                    println!("found literal: {word}");
                }
            }
        }
    }
}
