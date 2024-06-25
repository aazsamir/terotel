use app::{handle_events, Operation};
use crossterm::{
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use jaeger::{Jaeger, LookbackUnit};
use ratatui::prelude::*;
use std::io::{self, stdout, Error};
use ui::ui;
use clap::Parser;

pub mod app;
pub mod jaeger;
pub mod ui;
pub mod cli;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[clap(short, long, default_value = "http://localhost:16686")]
    url: String,
    #[clap(short, long, default_value = "false")]
    interactive: bool,
    #[clap(short, long)]
    service: Option<String>,
    #[clap(short, long)]
    operation: Option<String>,
    #[clap(long)]
    list_services: bool,
    #[clap(long)]
    list_operations: bool,
    #[clap(short, long, default_value = "plain")]
    format: String,
    #[clap(long)]
    lookback: Option<i32>,
    #[clap(long)]
    #[arg(value_enum)]
    lookback_unit: Option<LookbackUnit>,
    #[clap(long)]
    limit: Option<i32>,
    #[clap(long)]
    min_duration: Option<u64>,
    #[clap(long)]
    max_duration: Option<u64>,
}

fn main() {
    let args = Args::parse();
    let jaeger = Jaeger::new(&args.url);

    if args.interactive {
        interactive_ui(jaeger).unwrap();
    } else {
        cli(jaeger, args).unwrap();
    }
}

fn interactive_ui(jaeger: Jaeger) -> io::Result<()> {
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

fn cli(jaeger: Jaeger, args: Args) -> io::Result<()> {
    cli::CliHandler::new(jaeger).handle(args);

    Ok(())
}