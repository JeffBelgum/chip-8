use time::{now, Tm};

const TIMER_SPEED_NS: i64 = 1_000_000_000 / 60;

pub struct Timer {
    value: u8,
    at: Tm,

}

impl Timer {
    pub fn new() -> Timer {
        Timer {
            value: 0,
            at: now(),
        }
    }

    pub fn get_value(&self) -> u8 {
        self.value
    }

    pub fn set_value(&mut self, value: u8) {
        self.value = value;
    }

    pub fn cycle(&mut self) {
        if self.value > 0 {
            let current = now();
            let diff = (current - self.at).num_nanoseconds().unwrap();
            if diff > TIMER_SPEED_NS {
                self.value -= 1;
                self.at = current;
            }
        }
    }
}
