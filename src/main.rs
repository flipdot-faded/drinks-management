mod state;
mod error;

use std::io;
use std::io::prelude::*;

use state::*;
use error::*;

fn main() {
	let stdin = io::stdin();
	let mut state = State::Null;

	for line in stdin.lock().lines() {
		match process_line(line, &state) {
			Ok(new_state) => state = new_state,
			Err(e) => println!("{}", e)
		}
	}
}

fn process_line(line :io::Result<String>, state :&State) -> Result<State, ProcessError> {
	let ean = try!(line.map_err(ProcessError::IoErr));
	let len = ean.len();
	if len != 8 && len != 13 && len != 14 {
		return Err(ProcessError::EanLenErr(ean))
	}

	let ean_b = &ean.as_bytes();
	match ean_b[0] {
		b'2' => match ean_b[1] {
			b'3' => match ean_b[2] {
				b'0' => ean[len-2..len].parse::<u8>()
					.map_err(|_| ProcessError::BadEan(ean.clone()))
					.and_then(process_ctrl_card),
				b'1' => process_balance_card(&ean),
				_ => Err(ProcessError::UnknownMode(ean_b[2]))
			},
			_ => Err(ProcessError::BadEan(ean.clone())) // TODO
		},
		_ => match state {
			&State::Null => Ok(State::Null),
			_ => process_prod(&ean)
		}
	}
}

fn process_ctrl_card(num_bottles :u8) -> Result<State, ProcessError> {
	Ok(State::Null)
}

fn process_balance_card(ean :&str) -> Result<State, ProcessError> {
	Ok(State::Null)
}

fn process_prod(ean :&str) -> Result<State, ProcessError> {
	Ok(State::Null)
}
