use std::process::ExitCode;

use super::{
	commit_regex::CommitRegex, commit_retriever::CommitRetriever, options_loader::OptionsLoader,
};

pub struct App {
	commit_retriever: Box<dyn CommitRetriever>,
	options_loader: Box<dyn OptionsLoader>,
}

impl App {
	pub fn new(
		commit_retriever: Box<dyn CommitRetriever>,
		options_loader: Box<dyn OptionsLoader>,
	) -> Self {
		Self {
			commit_retriever,
			options_loader,
		}
	}

	pub fn run(&self) -> ExitCode {
		let options = match self.options_loader.load() {
			Ok(options) => options,
			Err(error) => {
				println!("Unable to load rummit options: {error}");
				return ExitCode::FAILURE;
			}
		};

		match options.is_valid() {
			Ok(_) => (),
			Err(error) => {
				println!("Invalid options: {error}");
				return ExitCode::FAILURE;
			}
		}

		if !options.enabled {
			return ExitCode::SUCCESS; // Do nothing.
		}

		let commit_regex = CommitRegex::from(&options);
		let commit_message = match self.commit_retriever.retrieve() {
			Ok(commit_message) => commit_message,
			Err(error) => {
				println!("Unable to get the commit message: {error}");
				return ExitCode::FAILURE;
			}
		};

		if !commit_regex.validate(&commit_message) {
			println!("Invalid commit message");
			return ExitCode::FAILURE;
		}

		if options.debug {
			println!("{} -> {}", commit_regex, commit_message);
			println!("Debug mod enabled, aborting commit...");
			return ExitCode::FAILURE;
		}
		ExitCode::SUCCESS
	}
}
