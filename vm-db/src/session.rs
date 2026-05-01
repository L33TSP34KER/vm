pub struct Session {
    pub active: bool,
}

impl Session {
    pub fn new() -> Self {
        Self { active: false }
    }

    pub fn attach(&mut self) {
        todo!("attach not yet implemented")
    }

    pub fn detach(&mut self) {
        todo!("detach not yet implemented")
    }
}
