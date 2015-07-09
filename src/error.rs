use std::{error, fmt, io, num};
use std::convert::From;

#[derive(Debug)]
pub enum ProcessError {
	IoErr(io::Error),
	EanParseErr(String),
	EanLenErr(u64)
}

impl fmt::Display for ProcessError {
	fn fmt(&self, f :&mut fmt::Formatter) -> Result<(), fmt::Error> {
		match self {
			&ProcessError::IoErr(_) => f.write_str("I/O error!"),
			&ProcessError::EanParseErr(ref s) => f.write_fmt(format_args!("Not an EAN code: \"{}\"", s)),
			&ProcessError::EanLenErr(code) => f.write_fmt(format_args!("Code is of wrong length: \"{}\"", code))
		}
	}
}

impl From<io::Error> for ProcessError {
	fn from(err :io::Error) -> ProcessError {
		ProcessError::IoErr(err)
	}
}
