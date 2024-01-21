#![allow(dead_code)]
use std::{process::exit, sync::atomic::Ordering};

use clap::{
    crate_authors, crate_version, value_parser, Arg, ArgAction,
    Command,
};
use log::{error, info, Level};
use rs_cms::state::DEFAULT_CONTEXT;

use helpers::*;
use lerp::*;

pub fn main() {
    #[allow(non_upper_case_globals)]
    let args = Command::new("Testbed")
        .author(crate_authors!("\n"))
        .version(crate_version!())
        .arg(
            Arg::new("checks")
                .action(ArgAction::SetFalse)
                .value_parser(value_parser!(bool))
                .default_value("true")
                .short('c')
                .long("checks")
                .help("Disables regular test/check functions"),
        )
        .arg(
            Arg::new("exhaustive")
                .action(ArgAction::SetTrue)
                .value_parser(value_parser!(bool))
                .default_value("false")
                .short('e')
                .long("exhaustive")
                .help("Enables exhaustive test functions"),
        )
        .arg(
            Arg::new("plugins")
                .action(ArgAction::SetTrue)
                .value_parser(value_parser!(bool))
                .default_value("false")
                .short('p')
                .long("plugins"),
        )
        .arg(
            Arg::new("speed")
                .action(ArgAction::SetTrue)
                .value_parser(value_parser!(bool))
                .default_value("false")
                .short('s')
                .long("speed"),
        )
        .arg(
            Arg::new("zoo")
                .action(ArgAction::SetTrue)
                .value_parser(value_parser!(bool))
                .default_value("false")
                .short('z')
                .long("zoo"),
        )
        .get_matches();

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

    if *args.get_one("checks").unwrap() {
        check("1D interpolation in 2pt tables", check_1d_lerp_2);
        check("1D interpolation in 3pt tables", check_1d_lerp_3);
        check("1D interpolation in 4pt tables", check_1d_lerp_4);
        check("1D interpolation in 6pt tables", check_1d_lerp_6);
        check("1D interpolation in 18pt tables", check_1d_lerp_18);
        check(
            "1D interpolation in descending 2pt tables",
            check_1d_lerp_2_down,
        );
        check(
            "1D interpolation in descending 3pt tables",
            check_1d_lerp_3_down,
        );
        check(
            "1D interpolation in descending 4pt tables",
            check_1d_lerp_4_down,
        );
        check(
            "1D interpolation in descending 6pt tables",
            check_1d_lerp_6_down,
        );
        check(
            "1D interpolation in descending 18pt tables",
            check_1d_lerp_18_down,
        );

        if *args.get_one("exhaustive").unwrap() {
            check("1D interpolation in n tables", exhaustive_check_1d_lerp);
            check(
                "1D interpolation in descending n tables",
                exhaustive_check_1d_lerp_down,
            );
        }

        check(
            "3D interpolation Tetrahedral (f32)",
            check_3d_interpolation_f32_tetrahedral,
        );
        check(
            "3D interpolation Trilinear (f32)",
            check_3d_interpolation_f32_trilinear,
        );
        check(
            "3D interpolation Tetrahedral (u16)",
            check_3d_interpolation_u16_tetrahedral,
        );
        check(
            "3D interpolation Trilinear (u16)",
            check_3d_interpolation_u16_trilinear,
        );

        if *args.get_one("exhaustive").unwrap() {
            check(
                "Exhaustive 3D interpolation Tetrahedral (f32)",
                exhaustive_check_3d_interpolation_f32_tetrahedral,
            );
            check(
                "Exhaustive 3D interpolation Trilinear (f32)",
                exhaustive_check_3d_interpolation_f32_trilinear,
            );
            check(
                "Exhaustive 3D interpolation Tetrahedral (u16)",
                exhaustive_check_3d_interpolation_u16_tetrahedral,
            );
            check(
                "Exhaustive 3D interpolation Trilinear (u16)",
                exhaustive_check_3d_interpolation_u16_trilinear,
            );
        }
    }

    let total_failed = TOTALFAIL.load(Ordering::SeqCst) as i32;
    info!("Total failed tests: {}", total_failed);

    exit(total_failed)
}

pub fn check(title: &str, test: TestFn) {
    info!("Checking {} ...", title);
    *REASON_TO_FAIL.lock().unwrap() = String::default();
    *SUBTEST.lock().unwrap() = String::default();
    TRAPPED_ERROR.store(false, Ordering::SeqCst);
    SIMULTANEOUS_ERRORS.store(0usize, Ordering::SeqCst);
    TOTALTESTS.fetch_add(1usize, Ordering::SeqCst);
    let val = test();
    if val.is_ok() && !TRAPPED_ERROR.load(Ordering::SeqCst) {
        info!("OK");
    } else {
        if let Err(msg) = val {
            error!("FAILED {}", msg);
        } else {
            error!("FAILED");
        }
        let subtest = SUBTEST.lock().unwrap();
        let reason_to_fail = REASON_TO_FAIL.lock().unwrap();
        if subtest.len() == 0 {
            error!("{}: [{}]\n\t{}", title, subtest, reason_to_fail);
        } else {
            error!("{}:\n\t{}", title, reason_to_fail);
        }
        TOTALFAIL.fetch_add(1, Ordering::SeqCst);
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

mod helpers;
mod lerp;
