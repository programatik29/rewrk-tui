use self::args::Args;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    prelude::{Constraint, CrosstermBackend, Direction, Layout},
    Frame, Terminal,
};
use state::State;
use std::{io::stdout, sync::Arc};

mod args;
mod bench;
mod state;
mod ui;

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let state = Arc::new(State::new(&args));

    bench::start(args, state.clone())?;

    enable_raw_mode()?;
    execute!(stdout(), EnterAlternateScreen, EnableMouseCapture)?;

    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    loop {
        terminal.draw(|f| ui(f, &state))?;

        if handle_events()? {
            break;
        }
    }

    disable_raw_mode()?;
    execute!(stdout(), LeaveAlternateScreen, DisableMouseCapture)?;

    Ok(())
}

fn ui(frame: &mut Frame, state: &Arc<State>) {
    let areas = Layout::new(
        Direction::Vertical,
        [
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(3),
        ],
    )
    .split(frame.size());

    ui::metrics::render(frame, state, areas[0]);
    ui::output::render(frame, state, areas[1]);
    ui::progress::render(frame, state, areas[2]);
}

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
