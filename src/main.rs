pub mod api;
pub mod counter;
pub mod dummy;
pub mod err;

use std::process::exit;
use std::thread;
use std::time::Duration;

use clap::{App, Arg};

use dummy::{start, Generator};
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
            Arg::with_name("metric_name")
                .short("N")
                .long("name")
                .default_value("dummy_time_ms")
                .help("Metric name"),
        )
        .arg(
            Arg::with_name("addr")
                .short("a")
                .long("addr")
                .default_value("0.0.0.0:9901")
                .help("Listhen address"),
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

    let generator = dummy::Generator::new(rate, mean, std_dev)?;

    if let Some(run_sec_str) = matches.value_of("test") {
        let run_sec: u64 = run_sec_str.parse()?;
        return test_run(generator, run_sec);
    }

    let addr = matches.value_of("addr").unwrap();
    let metric_name = matches.value_of("metric_name").unwrap();
    run_api(generator, addr, metric_name)
}

fn run_api(generator: Generator, addr: &str, metric_name: &str) -> Result<(), Error> {
    let counter = counter::Counter::new();
    start(generator, counter.clone());

    api::start_api(addr, counter, metric_name.to_owned());

    Ok(())
}

fn test_run(generator: Generator, run_sec: u64) -> Result<(), Error> {
    let counter = counter::Counter::new();
    start(generator, counter.clone());

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
