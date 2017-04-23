use std::{fmt, io};
use std::convert::From;
use std::path::PathBuf;
use std::string::FromUtf8Error;
use valueexpression::Value;

#[derive(Debug)]
pub enum Error {
    Input(PathBuf, io::Error),
    IoError(io::Error),
    Encoding(FromUtf8Error),
    BadValue(String),
    S(String),
}

impl Error {
    pub fn bad_value(expected: &str, actual: &Value) -> Self {
        Error::BadValue(format!("expected {}, got {} = {}",
                                expected,
                                actual.type_name(),
                                actual))
    }
}

impl fmt::Display for Error {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::S(ref s) => write!(out, "{}", s),
            Error::Input(ref p, ref e) => {
                write!(out, "Failed to read {:?}: {}", p, e)
            }
            // fallback
            ref x => write!(out, "{:?}", x),
        }
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error::IoError(e)
    }
}

impl From<FromUtf8Error> for Error {
    fn from(e: FromUtf8Error) -> Self {
        Error::Encoding(e)
    }
}