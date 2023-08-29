use std::error::Error;

pub trait CommitRetriever {
	fn retrieve(&self) -> Result<String, Box<dyn Error>>;
}
