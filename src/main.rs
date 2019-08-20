pub mod counter;
pub mod dummy;
pub mod err;

use std::process::exit;
use std::thread;
use std::time::Duration;

use clap::{App, Arg};

use err::Error;

fn main() {
    if let Err(e) = w_main() {
        eprintln!("Error: {}", e);
        exit(1);
    }
}

fn w_main() -> Result<(), Error> {
    let matches = App::new("envoy-dummy-stats")
        .arg(
            Arg::with_name("rate")
                .short("r")
                .long("rate")
                .default_value("50")
                .help("Entry per seconds"),
        )
        .arg(
            Arg::with_name("mean")
                .short("m")
                .long("mean")
                .default_value("20")
                .help("Mean of distribution. (ms)"),
        )
        .arg(
            Arg::with_name("stddev")
                .short("d")
                .long("stddev")
                .default_value("20")
                .help("Stddev of distribution. (ms)"),
        )
        .arg(
            Arg::with_name("test")
                .long("test")
                .takes_value(true)
                .help("Run specified seconds and dump counts."),
        )
        .get_matches();

    let rate: f64 = matches.value_of("rate").unwrap().parse()?;
    let mean: f64 = matches.value_of("mean").unwrap().parse()?;
    let std_dev: f64 = matches.value_of("stddev").unwrap().parse()?;

    if let Some(run_sec_str) = matches.value_of("test") {
        let run_sec: u64 = run_sec_str.parse()?;
        return test_run(rate, mean, std_dev, run_sec);
    }

    Ok(())
}

fn test_run(rate: f64, mean: f64, std_dev: f64, run_sec: u64) -> Result<(), Error> {
    let counter = counter::Counter::new();
    let generator = dummy::Generator::new(rate, mean, std_dev)?;

    dummy::start(generator, counter.clone());
    thread::sleep(Duration::from_secs(run_sec));
    let cnt = counter.get_count();

    println!("Count: {:.4}", cnt.count());
    println!("Sum:   {:.4}", cnt.sum());
    println!();

    for bc in cnt.buckets() {
        if bc.le < 1.0 {
            println!("<= {:7.1}: {:3} ({:3})", bc.le, bc.sum, bc.count);
        } else {
            println!("<= {:7.0}: {:3} ({:3})", bc.le, bc.sum, bc.count);
        }
    }

    Ok(())
}
