use app::{handle_events, Operation};
use crossterm::{
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use jaeger::Jaeger;
use ratatui::prelude::*;
use ui::ui;
use std::{
    io::{self, stdout, Error},
};

pub mod jaeger;
pub mod app;
pub mod ui;

fn main() -> io::Result<()> {
    let jaeger = Jaeger::new("http://localhost:16686");
    let mut state = app::State::new();
    let services = jaeger.get_services().unwrap();
    state.services = Some(services);
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    let mut operation: Result<Operation, Error>;
    let mut should_quit = false;

    while !should_quit {
        terminal.draw(|f| {
            ui(f, &state);
        })?;
        operation = handle_events(&state);

        if let Ok(op) = operation {
            state = state.handle_operation(&op, &jaeger);
        }

        should_quit = state.should_quit;
    }

    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}