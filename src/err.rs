use std::error;
use std::fmt;

#[derive(Debug)]
pub struct Error(Box<dyn error::Error>);

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}
