mod state;
mod error;

use std::io;
use std::io::prelude::*;

use state::*;
use error::*;

fn main() {
	let stdin = io::stdin();
	let state = State::Null;

	for line in stdin.lock().lines() {
		line.map(|code| process_code(code, &state)).map_err(|err|
			println!("{}", err)
		);
	}
}

fn process_code(code :String, state :&State) -> Result<(), ProcessError> {
	code.parse::<u64>()
		.map_err(|_| ProcessError::EanParseErr(code))
		.and_then(|ean| process_ean(ean, state))
}

fn process_ean(ean :u64, state :&State) -> Result<(), ProcessError> {
	Ok(())
}
