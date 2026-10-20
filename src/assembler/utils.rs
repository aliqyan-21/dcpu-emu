use std::fs;

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

pub fn read_asm(path: String) -> Vec<String> {
    let mut asm: Vec<String> = Vec::new();
    let asmc = fs::read_to_string(path).expect("file could not be opened: {path}");
    for line in asmc.lines() {
        let line = normalize(line);
        if !line.is_empty() {
            asm.push(line);
        }
    }
    asm
}
