use std::{env, error::Error};

use crate::rummit::commit_retriever::CommitRetriever;

pub struct GithookRetriever;

impl GithookRetriever {
	pub fn new() -> Self {
		Self
	}
}

impl CommitRetriever for GithookRetriever {
	fn retrieve(&self) -> Result<String, Box<dyn Error>> {
		let args: Vec<String> = env::args().collect();

		return match args.get(1) {
			Some(commit_message) => Ok(commit_message.to_owned()),
			None => Err("Unable to get the commit message from the githook.".into()),
		};
	}
}
