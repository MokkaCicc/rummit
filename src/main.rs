mod commit;

use commit::Commit;
use std::env;

fn main() {
	let commit = Commit::builder()
		.reverse()
		.optionnal()
		.types(vec!["feat", "fix"])
		.scopes()
		.optionnal()
		.breaking()
		.optionnal()
		.colon()
		.description(1, 52)
		.build();

	let args: Vec<String> = env::args().collect();
	let commit_message = args.get(1).unwrap();
	println!("{}", commit.regex);
	println!("{}", commit_message);
	println!("{}", commit.is_valid(commit_message.as_str()));
}
