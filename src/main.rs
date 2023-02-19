mod commit;

use commit::Commit;
use std::{env, process::ExitCode};

fn main() -> ExitCode {
	let commit = Commit::builder()
		.types(vec!["feat", "fix"])
		.colon()
		.description(1, 52)
		.build();

	let args: Vec<String> = env::args().collect();
	let commit_message = args.get(1).unwrap();
	println!("{} -> {}", commit.regex, commit_message);

	if !commit.validate(&commit_message) {
		println!("Invalid commit message");
		return ExitCode::FAILURE;
	}
	ExitCode::SUCCESS
}
