use app::{handle_events, Operation};
use clap::Parser;
use crossterm::{
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use jaeger::{Jaeger, JaegerService, ProtoService};
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
    // #[clap(short, long, default_value = "http://localhost:16685")]
    url: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let mut jaeger = JaegerService::new(&args.url);
    // let mut jaeger = ProtoService::new(&args.url)
    //     .await
    //     .expect("Failed to connect to Jaeger");

    let res = interactive(&mut jaeger).await;

    disable_raw_mode().expect("Failed to disable raw mode");
    stdout().execute(LeaveAlternateScreen).expect("Failed to leave alternate screen");

    if let Err(e) = res {
        eprintln!("Error: {}", e);
    }
}

async fn interactive(jaeger: &mut dyn Jaeger) -> io::Result<()> {
    let mut state = app::State::new();
    // todo: there may be nothing to unwrap here
    let services = jaeger.get_services().await;
    if let Ok(services) = services {
        state.services = Some(services);
    } else {
        // wait 1s
        std::thread::sleep(std::time::Duration::from_secs(1));
        // just retry, there will be traces now
        state.services = Some(
            jaeger
                .get_services()
                .await
                .expect("No traces available yet"),
        );
    }
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;

    // panic handler to change terminal mode back to normal, and print the error
    let _ = std::panic::take_hook();
    std::panic::set_hook(Box::new(|info| {
        disable_raw_mode().expect("Failed to disable raw mode");
        stdout().execute(LeaveAlternateScreen).expect("Failed to leave alternate screen");
        eprintln!("{}", info);
    }));

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
            state = state.handle_operation(&op, jaeger).await;

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
