pub struct Timer {
    value: u8
}

impl Timer {
    pub fn new() -> Timer {
        Timer {
            value: 0
        }
    }
    pub fn get_value(&self) -> u8 {
        self.value
    }
    pub fn set_value(&mut self, value: u8) {
        self.value = value;
    }
    pub fn decr(&mut self) {
        if self.value > 0 {
            self.value -= 1;
        }
    }
}
