pub struct MemoryView;

impl MemoryView {
    pub fn new() -> Self {
        Self
    }

    pub fn read_byte(&self, _address: usize) -> u8 {
        todo!("read_byte not yet implemented")
    }

    pub fn write_byte(&mut self, _address: usize, _value: u8) {
        todo!("write_byte not yet implemented")
    }
}
