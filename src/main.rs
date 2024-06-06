use app::{handle_events, Operation};
use crossterm::{
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use jaeger::Jaeger;
use ratatui::prelude::*;
use std::io::{self, stdout, Error};
use ui::ui;

pub mod app;
pub mod jaeger;
pub mod ui;

fn main() -> io::Result<()> {
    // grpc_main()?;
    let jaeger = Jaeger::new("http://localhost:16686");
    let mut state = app::State::new();
    // todo: there may be nothing to unwrap here
    let services = jaeger.get_services().unwrap();
    state.services = Some(services);
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    let mut operation: Result<Operation, Error>;
    let mut should_quit = false;

    // first draw
    terminal.draw(|f| {
        ui(f, &state);
    })?;

    while !should_quit {
        operation = handle_events(&state);

        if let Ok(op) = operation {
            state = state.handle_operation(&op, &jaeger);

            // draw only if there was any operation
            if op != Operation::Nothing {
                terminal.draw(|f| {
                    ui(f, &state);
                })?;
            }

            should_quit = state.should_quit;
        }
    }

    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}


// fn grpc_main() -> io::Result<()> {
//     tonic::include_proto!("jaeger.api_v2");
//     Ok(())
// }