use super::super::dcpu::dcpu::DCPU;
use super::device::Device;
use std::collections::VecDeque;

const BUFFER_CAP: usize = 64;
const KEY_SHIFT: u16 = 0x90;
const KEY_CTRL: u16 = 0x91;

#[derive(Clone, Copy)]
pub enum KeyEvent {
    Press(u16),
    Release(u16),
}

pub struct Keyboard {
    buffer: VecDeque<u16>,
    pressed: [bool; 0x92],
    int_msg: Option<u16>,
    pending: bool,
    script: VecDeque<(u64, KeyEvent)>,
}

fn with_shift(key: u16) -> u16 {
    if key > 0xFF {
        return key;
    }

    let c = key as u8;
    (match c {
        b'a'..=b'z' => c - 32,
        b'[' | b'\\' | b']' => c + 32,
        b'0' => b')',
        b'1' => b'!',
        b'2' => b'@',
        b'3' => b'#',
        b'4' => b'$',
        b'5' => b'%',
        b'6' => b'^',
        b'7' => b'&',
        b'8' => b'*',
        b'9' => b'(',
        b';' => b':',
        b'=' => b'+',
        b',' => b'<',
        b'.' => b'>',
        b'/' => b'?',
        b'\'' => b'"',
        b'`' => b'~',
        _ => c,
    } as u16)
}

impl Keyboard {
    pub fn new() -> Self {
        Self {
            buffer: VecDeque::new(),
            pressed: [false; 0x92],
            int_msg: None,
            pending: false,
            script: VecDeque::new(),
        }
    }

    pub fn press(&mut self, key: u16) {
        if key as usize >= self.pressed.len() {
            return;
        }

        if key != KEY_SHIFT && key != KEY_CTRL && self.buffer.len() < BUFFER_CAP {
            let typed = if self.pressed[KEY_SHIFT as usize] {
                with_shift(key)
            } else {
                key
            };
            self.buffer.push_back(typed);
        }
        self.pressed[key as usize] = true;
        self.pending = true;
    }

    pub fn with_script(mut events: Vec<(u64, KeyEvent)>) -> Self {
        events.sort_by_key(|(t, _)| *t);
        let mut kbd = Self::new();
        kbd.script = events.into();
        kbd
    }

    pub fn release(&mut self, key: u16) {
        if key as usize >= self.pressed.len() {
            return;
        }
        self.pressed[key as usize] = false;
        self.pending = true;
    }
}

impl Device for Keyboard {
    fn hardware_id(&self) -> u32 {
        0x30cf7406
    }
    fn version(&self) -> u16 {
        1
    }
    fn manufacturer(&self) -> u32 {
        0
    }

    fn interrupt(&mut self, dcpu: &mut DCPU) -> u32 {
        match dcpu.registers[0] {
            0 => self.buffer.clear(),
            1 => dcpu.registers[2] = self.buffer.pop_front().unwrap_or(0),
            2 => {
                let key = dcpu.registers[1] as usize;
                dcpu.registers[2] = self.pressed.get(key).copied().unwrap_or(false) as u16;
            }
            3 => {
                let b = dcpu.registers[1];
                self.int_msg = if b != 0 { Some(b) } else { None };
                self.pending = false;
            }
            _ => {}
        }
        0
    }

    fn tick(&mut self, now: u64) -> Option<u64> {
        while let Some(&(t, ev)) = self.script.front() {
            if t > now {
                break;
            }
            self.script.pop_front();
            match ev {
                KeyEvent::Press(k) => self.press(k),
                KeyEvent::Release(k) => self.release(k),
            }
        }
        if let Some(msg) = self.int_msg {
            if self.pending {
                self.pending = false;
                return Some(msg as u64);
            }
        }
        None
    }
}
