use super::CommitBuilder;
use regex::Regex;

pub struct Commit {
	pub regex: Regex,
}

impl Commit {
	pub fn builder() -> CommitBuilder {
		CommitBuilder::new()
	}

	pub fn is_valid(&self, commit: &str) -> bool {
		self.regex.is_match(commit)
	}
}
