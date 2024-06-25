use crate::cli::CliHandlerrer;
use clap::Parser;
use jaeger::{JaegerService, LookbackUnit};

pub mod cli;
pub mod jaeger;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[clap(short, long, default_value = "http://localhost:16686")]
    url: String,
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
    let jaeger = JaegerService::new(&args.url);
    cli(jaeger, args).unwrap();
}

fn cli(jaeger: JaegerService, args: Args) -> Result<(), ()> {
    let handler = cli::CliHandler::new(jaeger);
    handler.handle(args);

    Ok(())
}
