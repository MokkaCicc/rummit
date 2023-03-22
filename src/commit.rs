use crate::commit_builder::CommitBuilder;
use crate::rummit_options::{RummitOptions, ScopeType};
use regex::Regex;

pub struct Commit {
	pub regex: Regex,
}

impl Commit {
	pub fn builder() -> CommitBuilder {
		CommitBuilder::new()
	}

	pub fn from_options(options: &RummitOptions) -> Commit {
		let mut builder = Commit::builder();
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

	pub fn validate(&self, commit: &str) -> bool {
		self.regex.is_match(commit)
	}
}
