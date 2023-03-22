mod commit;
mod commit_builder;
mod rummit_options;

use commit::Commit;
use rummit_options::RummitOptions;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::{env, process::ExitCode};

const CONFIG_PATH: &str = "rummit.json";

fn main() -> ExitCode {
	let options = match read_config() {
		Ok(options) => options,
		Err(error) => {
			println!("Unable to parse config file: {}", error);
			return ExitCode::FAILURE;
		}
	};

	if !options.enabled {
		return ExitCode::SUCCESS; // Do nothing.
	}

	let commit = Commit::from_options(&options);
	let commit_message = get_commit_message();

	if !commit.validate(&commit_message) {
		println!("Invalid commit message");
		return ExitCode::FAILURE;
	}

	if options.debug {
		println!("{} -> {}", commit.regex, commit_message);
		println!("Debug mod enabled, aborting commit...");
		return ExitCode::FAILURE;
	}
	ExitCode::SUCCESS
}

fn read_config() -> Result<RummitOptions, Box<dyn Error>> {
	let file = File::open(CONFIG_PATH)?;
	let reader = BufReader::new(file);
	let options = serde_json::from_reader(reader)?;
	Ok(options)
}

fn get_commit_message() -> String {
	let args: Vec<String> = env::args().collect();
	args.get(1).unwrap().to_owned()
}
