use std::{
    mem::size_of,
    process::exit,
    sync::{
        atomic::{AtomicBool, AtomicUsize, Ordering},
        Mutex,
    },
};

use clap::Parser;
use log::{error, info, Level};
use rs_cms::{
    state::{
        default_error_handler_log_function, Context, ErrorCode, ErrorHandlerLogFunction,
        DEFAULT_CONTEXT,
    },
    types::Signature,
    S15Fixed16Number, U16Fixed16Number, U8Fixed8Number,
};

use helpers::*;

static REASON_TO_FAIL: Mutex<String> = Mutex::new(String::new());
static SUBTEST: Mutex<String> = Mutex::new(String::new());
static TRAPPED_ERROR: AtomicBool = AtomicBool::new(false);
static SIMULTANEOUS_ERRORS: AtomicUsize = AtomicUsize::new(0usize);
static TOTALTESTS: AtomicUsize = AtomicUsize::new(0usize);
static TOTALFAIL: AtomicUsize = AtomicUsize::new(0usize);

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

    print_supported_intents();

    check("Base types", check_base_types);
    check("Quick floor", check_quick_floor);
    check("Quick floor word", check_quick_floor_word);
}

fn check(title: &str, test: fn() -> bool) {
    info!("Checking {} ...", title);
    *REASON_TO_FAIL.lock().unwrap() = String::new();
    *SUBTEST.lock().unwrap() = String::new();
    TRAPPED_ERROR.store(false, Ordering::SeqCst);
    SIMULTANEOUS_ERRORS.store(0usize, Ordering::SeqCst);
    TOTALTESTS.fetch_add(1usize, Ordering::SeqCst);
    if test() {
        info!("OK");
    } else {
        error!("FAILED");
        TOTALFAIL.fetch_add(1, Ordering::SeqCst);
    }
}

fn die(context_id: &Context, level: Level, error_code: ErrorCode, text: &str) {
    default_error_handler_log_function(context_id, level, error_code, text);

    if level >= Level::Error {
        exit(1);
    }
}

fn print_supported_intents() {
    let mut intents: Vec<(u32, &str)> = Vec::with_capacity(200);

    let n = DEFAULT_CONTEXT.get_supported_intents(200, &mut intents);

    info!("Supported intents:");
    for i in 0..n {
        info!("\t{} - {}", intents[i].0, intents[i].1);
    }
}

fn check_base_types() -> bool {
    if size_of::<u8>() != 1 {
        return false;
    }
    if size_of::<i8>() != 1 {
        return false;
    }
    if size_of::<u16>() != 2 {
        return false;
    }
    if size_of::<i16>() != 2 {
        return false;
    }
    if size_of::<u32>() != 4 {
        return false;
    }
    if size_of::<i32>() != 4 {
        return false;
    }
    if size_of::<u64>() != 8 {
        return false;
    }
    if size_of::<i64>() != 8 {
        return false;
    }
    if size_of::<f32>() != 4 {
        return false;
    }
    if size_of::<f64>() != 8 {
        return false;
    }
    if size_of::<Signature>() != 4 {
        return false;
    }
    if size_of::<U8Fixed8Number>() != 2 {
        return false;
    }
    if size_of::<S15Fixed16Number>() != 4 {
        return false;
    }
    if size_of::<U16Fixed16Number>() != 4 {
        return false;
    }
    true
}

fn check_quick_floor() -> bool {
    if rs_cms::quick_floor(1.234) != 1 {
        return false;
    }
    if rs_cms::quick_floor(32767.234) != 32767 {
        return false;
    }
    if rs_cms::quick_floor(-1.234) != -2 {
        return false;
    }
    if rs_cms::quick_floor(-32767.1) != -32768 {
        return false;
    }

    true
}

fn check_quick_floor_word() -> bool {
    for i in 0..u16::MAX {
        if rs_cms::quick_floor_word(i as f64 + 0.1234) != i {
            die(
                &DEFAULT_CONTEXT,
                Level::Error,
                ErrorCode::NotSuitable,
                "quick_floor_word failed. Please use the \"no_fast_floor\" feature",
            );
            return false;
        }
    }
    true
}

const FIXED_PRECISION_15_16: f64 = 1.0 / 65535.0;
const FIXED_PRECISION_8_8: f64 = 1.0 / 255.0;
const FLOAT_PRECISION: f64 = 1.0e-5;
mod helpers;
