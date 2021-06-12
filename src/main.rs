mod digit;
mod timer;

use timer::Timer;

use std::io;
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};

use crossterm::event::{self, read, Event, KeyCode};
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use tui::layout::{Constraint, Layout};
use tui::widgets::{Block, Borders, Paragraph};
use tui::{backend::CrosstermBackend, Terminal};

enum AppEvent<I> {
    Input(I),
    Tick,
}

fn main() -> anyhow::Result<()> {
    enable_raw_mode()?;

    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let (tx, rx) = mpsc::channel();
    let tick_rate = Duration::from_secs(1);
    thread::spawn(move || {
        let mut last_tick = Instant::now();
        loop {
            // Poll for tick rate duration and check if there is an Event available
            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));
            if let Ok(true) = event::poll(timeout) {
                if let Ok(Event::Key(key)) = read() {
                    tx.send(AppEvent::Input(key)).unwrap();
                }
            }

            // If no key input event, send tick event
            if last_tick.elapsed() >= tick_rate {
                tx.send(AppEvent::Tick).unwrap();
                last_tick = Instant::now();
            }
        }
    });

    // let mut timer = Timer::new(60 * 18 + 11);
    let mut timer = Timer::new(3600 * 29 + 60 * 18 + 41);

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
            let paragraph = Paragraph::new(timer.text()).alignment(tui::layout::Alignment::Center);
            f.render_widget(paragraph, chunks[0]);
        })?;

        match rx.recv()? {
            AppEvent::Input(event) => match event.code {
                KeyCode::Char('q') => {
                    disable_raw_mode()?;
                    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
                    break;
                }
                _ => {}
            },
            AppEvent::Tick => {
                timer.tick();
            }
        }
    }

    Ok(())
}
