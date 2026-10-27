use crate::dcpu::dcpu::DCPU;
use crate::devices::clock::Clock;
use crate::devices::keyboard::*;
use std::fs;
use std::path::Path;

fn parse_num(s: &str) -> Option<u64> {
    match s.strip_prefix("0x") {
        Some(h) => u64::from_str_radix(h, 16).ok(),
        None => s.parse().ok(),
    }
}

fn parse_expect(src: &str) -> Vec<(String, u64)> {
    let mut out = Vec::new();
    for line in src.lines() {
        let line = line.trim();
        if !line.starts_with(';') {
            break; // header should be first comment
        }
        if let Some(rest) = line.trim_start_matches(';').trim().strip_prefix("expect:") {
            for tok in rest.split_whitespace() {
                let (k, v) = tok.split_once('=').expect("bad expect token");
                out.push((k.to_lowercase(), parse_num(v).expect("bad expect value")));
            }
        }
    }
    out
}

fn parse_devices(src: &str) -> Vec<String> {
    let mut out = Vec::new();
    for line in src.lines() {
        let line = line.trim();
        if !line.starts_with(';') {
            break;
        }
        if let Some(rest) = line.trim_start_matches(';').trim().strip_prefix("devices:") {
            out.extend(rest.split_whitespace().map(|s| s.to_lowercase()));
        }
    }
    out
}

fn parse_keys(src: &str) -> Vec<(u64, KeyEvent)> {
    let mut out = Vec::new();
    for line in src.lines() {
        let line = line.trim();
        if !line.starts_with(';') {
            break;
        }
        if let Some(rest) = line.trim_start_matches(';').trim().strip_prefix("keys:") {
            for tok in rest.split_whitespace() {
                let (t, ev) = tok.split_once(':').expect("bad keys token");
                let time: u64 = t.parse().expect("bad cycle");
                let (sign, key) = ev.split_at(1);
                let key = parse_num(key).expect("bad key") as u16;
                let event = match sign {
                    "+" => KeyEvent::Press(key),
                    "-" => KeyEvent::Release(key),
                    _ => panic!("key event must start with + or -"),
                };
                out.push((time, event));
            }
        }
    }
    out
}

fn actual(cpu: &DCPU, key: &str) -> Option<u64> {
    Some(match key {
        "a" => cpu.registers[0] as u64,
        "b" => cpu.registers[1] as u64,
        "c" => cpu.registers[2] as u64,
        "x" => cpu.registers[3] as u64,
        "y" => cpu.registers[4] as u64,
        "z" => cpu.registers[5] as u64,
        "i" => cpu.registers[6] as u64,
        "j" => cpu.registers[7] as u64,
        "pc" => cpu.pc as u64,
        "sp" => cpu.sp as u64,
        "ex" => cpu.ex as u64,
        "ia" => cpu.ia as u64,
        "cycles" => cpu.cycles,
        _ => return None,
    })
}

fn run_file(path: &Path) -> Result<bool, String> {
    let src = fs::read_to_string(path).map_err(|e| e.to_string())?;
    let expected = parse_expect(&src);
    if expected.is_empty() {
        return Ok(false);
    }

    let mut dcpu = DCPU::new();
    let keys = parse_keys(&src);
    for name in parse_devices(&src) {
        match name.as_str() {
            "clock" => dcpu.devices.push(Box::new(Clock::new())),
            "keyboard" => dcpu
                .devices
                .push(Box::new(Keyboard::with_script(keys.clone()))),
            other => return Err(format!("unknown device `{other}`")),
        }
    }
    dcpu.load_instructions(path.to_str().unwrap())?;
    dcpu.dcpu_run(false)?;

    let mut errs = Vec::new();
    for (k, want) in expected {
        match actual(&dcpu, &k) {
            Some(got) if got == want => {}
            Some(got) => errs.push(format!("{k}: expected 0x{want:x}, got 0x{got:x}")),
            None => errs.push(format!("unknown key `{k}`")),
        }
    }
    if errs.is_empty() {
        Ok(true)
    } else {
        Err(errs.join("; "))
    }
}

#[test]
fn corpus() {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("corpus");
    let mut files: Vec<_> = fs::read_dir(&dir)
        .unwrap()
        .map(|e| e.unwrap().path())
        .filter(|p| p.extension().map_or(false, |e| e == "s"))
        .collect();
    files.sort();

    let mut failures = Vec::new();
    let mut ran = 0;
    for f in &files {
        match run_file(f) {
            Ok(true) => ran += 1,
            Ok(false) => println!(
                "skipped (no header): {}",
                f.file_name().unwrap().to_string_lossy()
            ),
            Err(e) => failures.push(format!(
                "{}: {}",
                f.file_name().unwrap().to_string_lossy(),
                e
            )),
        }
    }
    assert!(
        failures.is_empty(),
        "corpus failures:\n{}",
        failures.join("\n")
    );
    assert!(
        ran > 0,
        "no corpus files with an `; expect:` header were found"
    );
}
