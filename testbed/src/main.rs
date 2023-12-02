use std::{process::exit, sync::atomic::Ordering};

use clap::Parser;
use log::{info, Level};
use rs_cms::state::DEFAULT_CONTEXT;

use helpers::*;
use lerp::*;

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

fn print_supported_intents() {
    let mut intents: Vec<(u32, &str)> = Vec::with_capacity(200);

    let n = DEFAULT_CONTEXT.get_supported_intents(200, &mut intents);

    info!("Supported intents:");
    for i in 0..n {
        info!("\t{} - {}", intents[i].0, intents[i].1);
    }
}

pub fn main() {
    let args = Cli::parse();

    simple_logger::init_with_level(Level::Info).unwrap();

    info!("rs-cms {} test bed", rs_cms::VERSION);

    info!("Installing error logger ...");
    DEFAULT_CONTEXT.set_error_logger(Some(fatal_error_quit));
    info!("done.");

    print_supported_intents();

    check("Base types", check_base_types);
    check("Quick floor", check_quick_floor);
    check("Quick floor word", check_quick_floor_word);
    check("Fixed point 15.16 representation", check_fixed_point_15_16);
    check("Fixed point 8.8 representation", check_fixed_point_8_8);
    check("D50 roundtrip", check_d50_roundtrip);

    exit(TOTALFAIL.load(Ordering::SeqCst) as i32)
}

mod helpers;
mod lerp;
