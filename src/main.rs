use std::io;

use crossterm::event::{read, Event, KeyCode};
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use tui::layout::{Constraint, Layout};
use tui::widgets::{Block, Borders, Paragraph};
use tui::{backend::CrosstermBackend, Terminal};

fn main() -> anyhow::Result<()> {
    enable_raw_mode()?;

    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    loop {
        terminal.draw(|f| {
            let size = f.size();

            // Surrounding block
            let block = Block::default().borders(Borders::ALL);
            f.render_widget(block, size);

            // Body
            let chunks = Layout::default()
                .direction(tui::layout::Direction::Vertical)
                .margin(2)
                .constraints([Constraint::Percentage(100)].as_ref())
                .split(f.size());
            let paragraph = Paragraph::new("Hello").alignment(tui::layout::Alignment::Center);
            f.render_widget(paragraph, chunks[0]);
        })?;
        let event = read().unwrap();
        if event == Event::Key(KeyCode::Char('q').into()) {
            disable_raw_mode()?;
            execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
            break;
        }
    }

    Ok(())
}
