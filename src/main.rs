#![warn(missing_docs)]

pub mod counter;
pub mod err;

use std::process::exit;

use err::Error;

fn main() {
    if let Err(e) = w_main() {
        eprintln!("Error: {}", e);
        exit(1);
    }
}

fn w_main() -> Result<(), Error> {
    Ok(())
}
