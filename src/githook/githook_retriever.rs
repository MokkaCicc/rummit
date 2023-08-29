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
		Ok(args.get(1).unwrap().to_owned())
	}
}
