use std::error;
use std::fmt;
use std::num::{ParseFloatError, ParseIntError};

use rand_distr::{ExpError, NormalError};

#[derive(Debug)]
pub struct Error(Box<dyn error::Error>);

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl error::Error for Error {}

#[derive(Debug)]
pub struct WrapError(Box<dyn fmt::Debug>);

impl fmt::Display for WrapError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl error::Error for WrapError {}

macro_rules! impl_error_from {
    ($name: ty) => {
        impl From<$name> for Error {
            fn from(e: $name) -> Error {
                Error(Box::new(e))
            }
        }
    };
}

impl_error_from!(ParseFloatError);
impl_error_from!(ParseIntError);

macro_rules! impl_error_from_wrap {
    ($name: ty) => {
        impl From<$name> for Error {
            fn from(e: $name) -> Error {
                Error(Box::new(WrapError(Box::new(e))))
            }
        }
    };
}

impl_error_from_wrap!(NormalError);
impl_error_from_wrap!(ExpError);
