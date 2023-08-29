use super::commit_regex::CommitRegex;
use regex::Regex;

pub struct CommitRegexBuilder {
	regex_string: String,
}

impl CommitRegexBuilder {
	pub fn new() -> Self {
		Self {
			regex_string: String::from('^'),
		}
	}

	pub fn build(mut self) -> CommitRegex {
		self.regex_string.push('$');
		CommitRegex {
			regex: Regex::new(self.regex_string.as_str()).unwrap(),
		}
	}

	pub fn optionnal(mut self) -> Self {
		self.regex_string.push('?');
		self
	}

	pub fn reverse(mut self) -> Self {
		let expression = CommitRegexBuilder::group("revert: ");
		self.regex_string.push_str(&expression);
		self
	}

	pub fn types(mut self, types: &[String]) -> Self {
		let expression = CommitRegexBuilder::group(&types.join("|"));
		self.regex_string.push_str(&expression);
		self
	}

	pub fn value_scope(mut self, values: &[String]) -> Self {
		let expression = CommitRegexBuilder::group(&values.join("|"));
		self.regex_string.push_str(&expression);
		self
	}

	pub fn project_item_scope(mut self) -> Self {
		let expression = CommitRegexBuilder::group(r"\(#[\d]+\)");
		self.regex_string.push_str(&expression);
		self
	}

	pub fn flexible_scope(mut self) -> Self {
		let expression = CommitRegexBuilder::group(r"\(.+\)");
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
		let expression = format!(".{{{min},{max}}}");
		self.regex_string.push_str(&expression);
		self
	}

	fn group(expression: &str) -> String {
		format!("({expression})")
	}
}
