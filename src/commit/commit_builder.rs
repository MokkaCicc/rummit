use super::Commit;
use regex::Regex;

pub struct CommitBuilder {
	regex_string: String,
}

impl CommitBuilder {
	pub fn new() -> Self {
		Self {
			regex_string: String::from('^'),
		}
	}

	pub fn build(mut self) -> Commit {
		self.regex_string.push('$');
		Commit {
			regex: Regex::new(self.regex_string.as_str()).unwrap(),
		}
	}

	pub fn optionnal(mut self) -> Self {
		self.regex_string.push('?');
		self
	}

	pub fn reverse(mut self) -> Self {
		let expression = CommitBuilder::group("revert: ");
		self.regex_string.push_str(&expression);
		self
	}

	pub fn types(mut self, types: Vec<&str>) -> Self {
		let expression = CommitBuilder::group(&types.join("|"));
		self.regex_string.push_str(&expression);
		self
	}

	pub fn scopes(mut self) -> Self {
		let expression = CommitBuilder::group(r"\(.+\)");
		self.regex_string.push_str(&expression);
		self
	}

	pub fn breaking(mut self) -> Self {
		self.regex_string.push('!');
		self
	}

	pub fn colon(mut self) -> Self {
		self.regex_string.push_str(": ");
		self
	}

	pub fn description(mut self, min: i32, max: i32) -> Self {
		let expression = format!(".{{{},{}}}", min, max);
		self.regex_string.push_str(&expression);
		self
	}

	fn group(expression: &str) -> String {
		format!("({})", expression)
	}
}
