use std::{error, fmt, io, num};
use std::convert::From;

use postgres;

#[derive(Debug)]
pub enum ProcessError {
	IoErr(io::Error),
	EanLenErr(String),
	BadEan(String),
	UnknownMode(u8)
}

#[derive(Debug)]
pub enum DbError {
	PostgresError(postgres::error::Error),
	NoData(String)
}

impl fmt::Display for ProcessError {
	fn fmt(&self, f :&mut fmt::Formatter) -> Result<(), fmt::Error> {
		match self {
			&ProcessError::IoErr(_) => f.write_str("I/O error!"),
			&ProcessError::EanLenErr(ref ean) => f.write_fmt(
				format_args!("EAN is of wrong length ({}): \"{}\"", ean.len(), ean)),
			&ProcessError::BadEan(ref ean) => f.write_fmt(
				format_args!("EAN not recognized: {}", ean)),
			&ProcessError::UnknownMode(z) => f.write_fmt(
				format_args!("Unknown mode: {}", z))
		}
	}
}

impl From<io::Error> for ProcessError {
	fn from(err :io::Error) -> ProcessError {
		ProcessError::IoErr(err)
	}
}
