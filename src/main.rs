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
		process_line(line, &mut state).err().map(|err|
			println!("{}", err)
		);
	}
}

fn process_line(line :io::Result<String>, state :&mut State) -> Result<(), ProcessError> {
	let ean = try!(line.map_err(ProcessError::IoErr));
	Ok(())
}
