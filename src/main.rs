use clap::Parser;
use app::{handle_events, Operation};
use crossterm::{
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use jaeger::{Jaeger, JaegerService};
use ratatui::prelude::*;
use std::io::{self, stdout, Error};
use ui::ui;

pub mod app;
pub mod jaeger;
pub mod ui;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[clap(short, long, default_value = "http://localhost:16686")]
    url: String,
}

fn main() {
    let args = Args::parse();
    let jaeger = JaegerService::new(&args.url);
    interactive(&jaeger).unwrap();
}

fn interactive(jaeger: &dyn Jaeger) -> io::Result<()> {
    let mut state = app::State::new();
    // todo: there may be nothing to unwrap here
    let services = jaeger.get_services();
    if let Ok(services) = services {
        state.services = Some(services);
    } else {
        // wait 1s
        std::thread::sleep(std::time::Duration::from_secs(1));
        // just retry, there will be traces now
        state.services = Some(jaeger.get_services().expect("No traces available yet"));
    }
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
            state = state.handle_operation(&op, jaeger);

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
