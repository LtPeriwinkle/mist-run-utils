// msf split file parsing and Run utilities
use crate::run::Run;
use ron::de::from_reader;
use std::fs::OpenOptions;
impl Run {
        /// Create a run from an MSF (aka mist split file). If the file is malformed or a file error occurs, return None.
        /// Creates the file if it does not exist so it can be saved to when the timer is closed.
	pub fn from_msf_file(filename: &str) -> Option<Run> {
    		let file: std::fs::File;
    		match OpenOptions::new().read(true).create(true).open(filename) {
			Ok(x) => {file = x;}
			Err(_) => {return None;}
    		}
		let run = from_reader(&file);
		run.ok()
	}
}
