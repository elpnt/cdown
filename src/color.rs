use tui::style::Color::{self, *};

pub const COLOR_NAMES: [&str; 15] = [
    "black",
    "white",
    "red",
    "green",
    "yellow",
    "blue",
    "magenta",
    "cyan",
    "darkgray",
    "lightred",
    "lightgreen",
    "lightyellow",
    "lightblue",
    "lightmagenta",
    "lightcyan",
];

pub const TUI_COLORS: [Color; 15] = [
    Black,
    White,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    DarkGray,
    LightRed,
    LightGreen,
    LightYellow,
    LightBlue,
    LightMagenta,
    LightCyan,
];

pub fn color(input: &str) -> Color {
    for (i, &name) in COLOR_NAMES.iter().enumerate() {
        if input == name {
            return TUI_COLORS[i];
        }
    }
    LightBlue
}
