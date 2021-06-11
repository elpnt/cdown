use crate::digit::digit;

pub struct Timer {
    // in seconds
    pub remain: u64,
}

impl Timer {
    pub fn new(remain: u64) -> Timer {
        Timer { remain }
    }

    pub fn tick(&mut self) {
        self.remain -= 1;
    }

    pub fn hms(&self) -> (u64, u64, u64) {
        let h = self.remain / 3600;
        let m = (self.remain % 3600) / 60;
        let s = self.remain % 60;
        (h, m, s)
    }
}
