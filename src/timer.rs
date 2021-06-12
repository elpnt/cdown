use crate::digit::digit;
use tui::text::Spans;
pub struct Timer {
    remain: u32, // in seconds
}

impl Timer {
    pub fn new(remain: u32) -> Timer {
        Timer { remain }
    }

    pub fn tick(&mut self) {
        self.remain -= 1;
    }

    pub fn text(&self) -> Vec<Spans> {
        let mut lines = vec![String::default(); 5];
        let (h, m, s) = self.hms();
        // Show hours only when the first input time is 1hour or more
        if h > 0 {
            self.push_number(h, &mut lines, true);
            self.push_digit(':', &mut lines);
        }

        self.push_number(m, &mut lines, false);
        self.push_digit(':', &mut lines);
        self.push_number(s, &mut lines, false);

        lines.into_iter().map(Spans::from).collect::<Vec<Spans>>()
    }

    fn push_number(&self, num: u32, lines: &mut Vec<String>, is_hour: bool) {
        let num = if is_hour {
            num.to_string()
        } else {
            format!("{:02}", num)
        };
        let mut chars = num.chars().peekable();
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

    fn hms(&self) -> (u32, u32, u32) {
        let h = self.remain / 3600;
        let m = (self.remain % 3600) / 60;
        let s = self.remain % 60;
        (h, m, s)
    }
}
