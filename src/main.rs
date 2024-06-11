use clap::Parser;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{prelude::CrosstermBackend, Frame, Terminal};
use std::{io::stdout, thread};

mod bench;

#[derive(Debug, Parser)]
struct Args {
    /// Thread count [default: CPU core count]
    #[arg(short, long)]
    threads: Option<usize>,

    /// Connection count [default: Thread count x20]
    #[arg(short, long)]
    connections: Option<usize>,

    /// Benchmark duration in seconds
    #[arg(short, long)]
    duration: u64,

    /// Target URL
    target: String,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    enable_raw_mode()?;
    execute!(stdout(), EnterAlternateScreen, EnableMouseCapture)?;

    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    thread::spawn(|| bench::start(args));

    loop {
        terminal.draw(ui)?;

        if handle_events()? {
            break;
        }
    }

    disable_raw_mode()?;
    execute!(stdout(), LeaveAlternateScreen, DisableMouseCapture)?;

    Ok(())
}

fn ui(_frame: &mut Frame) {}

fn handle_events() -> std::io::Result<bool> {
    if event::poll(std::time::Duration::from_millis(250))? {
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press && key.code == KeyCode::Char('q') {
                return Ok(true);
            }
        }
    }
    Ok(false)
}
