use tui::layout::{Constraint, Direction, Layout, Rect};

pub fn center_area(r: Rect, content_height: u16, content_width: u16) -> Rect {
    let mh = margin(r.height, content_height);
    let mw = margin(r.width, content_width);
    let area = Layout::default()
        .horizontal_margin(1)
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(mh),
                Constraint::Length(content_height),
                Constraint::Length(mh),
            ]
            .as_ref(),
        )
        .split(r)[1];
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Length(mw),
                Constraint::Length(content_width),
                Constraint::Length(mw),
            ]
            .as_ref(),
        )
        .split(area)[1]
}

fn margin(rect_length: u16, content_length: u16) -> u16 {
    if rect_length < content_length {
        0
    } else {
        (rect_length - content_length) / 2
    }
}
