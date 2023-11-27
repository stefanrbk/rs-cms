use clap::Parser;
use log::{info, Level};

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

pub fn main() {
    let args = Cli::parse();

    simple_logger::init_with_level(Level::Info).unwrap();

    info!("rs-cms {} test bed", rs_cms::VERSION);
}
