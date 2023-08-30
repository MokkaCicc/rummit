use std::fmt::{self, Display, Formatter};

use super::commit_regex_builder::CommitRegexBuilder;
use super::options::{RummitOptions, ScopeType};
use regex::Regex;

pub struct CommitRegex {
	regex: Regex,
}

impl CommitRegex {
	pub fn builder() -> CommitRegexBuilder {
		CommitRegexBuilder::new()
	}

	pub fn validate(&self, commit: &str) -> bool {
		self.regex.is_match(commit)
	}
}

impl Display for CommitRegex {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		write!(f, "{}", self.regex)
	}
}

impl From<Regex> for CommitRegex {
	fn from(regex: Regex) -> Self {
		Self { regex }
	}
}

impl From<&RummitOptions> for CommitRegex {
	fn from(options: &RummitOptions) -> Self {
		let mut builder = CommitRegex::builder();
		match options.is_valid() {
			Ok(_) => (),
			Err(_) => return builder.build(),
		}

		if options.allow.revert {
			builder = builder.reverse();
			builder = builder.optionnal();
		}
		builder = builder.types(&options.types);
		// TODO: Verify that the scope values are here when the value scope is choosen.
		match options.scope._type {
			ScopeType::Value => {
				builder = builder.value_scope(options.scope.values.as_ref().unwrap())
			}
			ScopeType::ProjectItem => builder = builder.project_item_scope(),
			ScopeType::Flexible => builder = builder.flexible_scope(),
		};
		if !options.scope.required {
			builder = builder.optionnal();
		}
		if options.allow.breaking_change {
			builder = builder.breaking();
			builder = builder.optionnal();
		}
		builder = builder.colon();
		builder = builder.description(
			options.description_length.min,
			options.description_length.max,
		);

		builder.build()
	}
}
