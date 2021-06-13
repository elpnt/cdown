use crate::digit::digit;
use tui::text::Spans;
pub struct Timer {
    pub is_paused: bool,
    pub duration: u64, // in seconds
    show_hour: bool,
}

impl Timer {
    pub fn new(duration: u64) -> Timer {
        Timer {
            is_paused: false,
            duration,
            show_hour: duration >= 3600,
        }
    }

    pub fn tick(&mut self) {
        self.duration -= 1;
    }

    pub fn toggle(&mut self) {
        self.is_paused = !self.is_paused;
    }

    pub fn text(&self) -> Vec<Spans> {
        let mut lines = vec![String::default(); 5];
        let (h, m, s) = self.hms();

        if self.show_hour {
            self.push_number(h, &mut lines);
            self.push_digit(':', &mut lines);
        }
        self.push_number(m, &mut lines);
        self.push_digit(':', &mut lines);
        self.push_number(s, &mut lines);

        lines.into_iter().map(Spans::from).collect::<Vec<Spans>>()
    }

    fn push_number(&self, num: u64, lines: &mut Vec<String>) {
        let s = format!("{:02}", num);
        let mut chars = s.chars().peekable();
        while let Some(ch) = chars.next() {
            self.push_digit(ch, lines);
            if chars.peek().is_some() {
                self.push_space(lines);
            }
        }
    }

    fn push_digit(&self, ch: char, lines: &mut Vec<String>) {
        for (i, &line) in digit(ch).iter().enumerate() {
            let mut s = String::default();
            for &v in line.iter() {
                if v == 1 {
                    s.push('█');
                } else {
                    s.push(' ');
                }
            }
            lines[i].push_str(&s);
        }
    }

    fn push_space(&self, lines: &mut Vec<String>) {
        for line in lines.iter_mut() {
            line.push(' ');
        }
    }

    fn hms(&self) -> (u64, u64, u64) {
        let h = self.duration / 3600;
        let m = (self.duration % 3600) / 60;
        let s = self.duration % 60;
        (h, m, s)
    }
}
