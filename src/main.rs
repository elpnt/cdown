mod digit;
mod timer;
mod utils;

use std::io;
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};
use structopt::StructOpt;
use timer::Timer;
use utils::center_area;

use crossterm::event::{self, read, Event, KeyCode};
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use tui::layout::Alignment;
use tui::style::{Color, Style};
use tui::widgets::{Block, Borders, Clear, Paragraph};
use tui::{backend::CrosstermBackend, Terminal};

enum AppEvent<I> {
    Input(I),
    Tick,
}

#[derive(StructOpt)]
#[structopt(name = "countdown", about = "Simple timer app on terminal")]
struct Opt {
    #[structopt(default_value = "1m")]
    time: String,
}

fn main() -> anyhow::Result<()> {
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

    let opt = Opt::from_args();
    let duration: Duration = opt
        .time
        .parse::<humantime::Duration>()
        .unwrap_or_else(|e| {
            println!("Error: {}", e);
            std::process::exit(1)
        })
        .into();
    let mut timer = Timer::new(duration.as_secs());

    enable_raw_mode()?;
    loop {
        terminal.draw(|f| {
            let size = f.size();

            // Surrounding block
            let block = Block::default().borders(Borders::ALL);
            f.render_widget(block.clone(), size);

            // Timer display
            let display_area = center_area(size, 5, size.width);
            let timer_text = Paragraph::new(timer.text()).alignment(Alignment::Center);

            // Paused popout
            let popout_area = center_area(size, 3, 12);
            let pause_message = Paragraph::new(" ⏸ Paused ")
                .block(block.clone())
                .style(Style::default().fg(Color::White).bg(Color::DarkGray))
                .alignment(Alignment::Center);

            if timer.is_paused() {
                f.render_widget(Clear, popout_area);
                f.render_widget(block, size);
                f.render_widget(timer_text, display_area);
                f.render_widget(pause_message, popout_area);
            } else {
                f.render_widget(Clear, popout_area);
                f.render_widget(block, size);
                f.render_widget(timer_text, display_area);
            }
        })?;

        match rx.recv()? {
            AppEvent::Input(event) => match event.code {
                KeyCode::Char('q') | KeyCode::Esc => {
                    disable_raw_mode()?;
                    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
                    std::process::exit(0);
                }
                KeyCode::Char('p') => {
                    timer.toggle();
                }
                _ => {}
            },
            AppEvent::Tick => {
                if !timer.is_paused() {
                    if timer.duration() == 0 {
                        disable_raw_mode()?;
                        execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
                        break;
                    }

                    timer.tick();
                }
            }
        }
    }

    Ok(())
}
