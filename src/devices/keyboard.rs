use super::super::dcpu::dcpu::DCPU;
use super::device::Device;
use std::collections::VecDeque;

const BUFFER_CAP: usize = 64;
const KEY_SHIFT: u16 = 0x90;
const KEY_CTRL: u16 = 0x91;

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
        0
    }

    fn tick(&mut self, _now: u64) -> Option<u64> {
        None
    }
}
