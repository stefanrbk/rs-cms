use std::process::exit;

use clap::Parser;
use log::{info, Level};
use rs_cms::state::{
    default_error_handler_log_function, Context, ErrorCode, ErrorHandlerLogFunction,
};

#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    #[arg(short, long, default_value = "true")]
    checks: bool,
    #[arg(short, long, default_value = "false")]
    exhaustive: bool,
    #[arg(short, long, default_value = "false")]
    plugins: bool,
    #[arg(short, long, default_value = "false")]
    speed: bool,
    #[arg(short, long, default_value = "false")]
    zoo: bool,
}

const FATAL_ERROR_QUIT: Option<ErrorHandlerLogFunction> = Some(die);

pub fn main() {
    let args = Cli::parse();

    simple_logger::init_with_level(Level::Info).unwrap();

    info!("rs-cms {} test bed", rs_cms::VERSION);

    info!("Installing error logger ...");
    rs_cms::state::DEFAULT_CONTEXT.set_error_logger(FATAL_ERROR_QUIT);
    info!("done.");
}

fn die(context_id: &Context, level: Level, error_code: ErrorCode, text: &str) {
    default_error_handler_log_function(context_id, level, error_code, text);

    if level >= Level::Error {
        exit(1);
    }
}
