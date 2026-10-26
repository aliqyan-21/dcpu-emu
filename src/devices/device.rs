use super::super::dcpu::dcpu::DCPU;

pub trait Device {
    fn hardware_id(&self) -> u32;
    fn version(&self) -> u16;
    fn manufacturer(&self) -> u32;

    // HWI will call this, return cycles it cost
    fn interrupt(&mut self, dcpu: &mut DCPU) -> u32;

    // after every instruction we tick with current cycle count its
    // paramter, it could return Some(msg) for an interrupt
    fn tick(&mut self, _now: u64) -> Option<u64> {
        None
    }
}
