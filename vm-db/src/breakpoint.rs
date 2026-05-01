pub struct Breakpoint {
    pub address: usize,
    pub hit_count: u64,
    pub enabled: bool,
}

impl Breakpoint {
    pub fn new(address: usize) -> Self {
        Self {
            address,
            hit_count: 0,
            enabled: true,
        }
    }

    pub fn hit(&mut self) {
        self.hit_count += 1;
    }

    pub fn toggle(&mut self) {
        self.enabled = !self.enabled;
    }
}
