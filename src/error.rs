use std::{error, fmt, io, num};
use std::convert::From;

#[derive(Debug)]
pub enum ProcessError {
	EanParseErr(String),
	EanLenErr(u64),
	EanChecksumErr(u64),
	IoErr(io::Error)
}

impl From<io::Error> for ProcessError {
	fn from(err :io::Error) -> ProcessError {
		ProcessError::IoErr(err)
	}
}
