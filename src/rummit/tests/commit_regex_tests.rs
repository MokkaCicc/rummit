use crate::rummit::commit_regex::CommitRegex;
use crate::rummit::options::*;

#[test]
fn conventional_commit() {
	let options = conventional_commit_options();

	let commit_regex = CommitRegex::from(&options);

	assert_eq!(
		commit_regex.to_string(),
		r"^(revert: )?(feat|fix)(\(.+\))?!?: .{1,52}$"
	);
}

#[test]
fn conventional_commit_without_revert() {
	let options = conventional_commit_options();
	let mutated_options = RummitOptions {
		allow: AllowOptions {
			revert: false,
			breaking_change: options.allow.breaking_change,
		},
		..options
	};

	let commit_regex = CommitRegex::from(&mutated_options);

	assert_eq!(
		commit_regex.to_string(),
		r"^(feat|fix)(\(.+\))?!?: .{1,52}$"
	)
}

#[test]
fn conventional_commit_without_breaking() {
	let options = conventional_commit_options();
	let mutated_options = RummitOptions {
		allow: AllowOptions {
			revert: options.allow.revert,
			breaking_change: false,
		},
		..options
	};

	let commit_regex = CommitRegex::from(&mutated_options);

	assert_eq!(
		commit_regex.to_string(),
		r"^(revert: )?(feat|fix)(\(.+\))?: .{1,52}$"
	)
}

#[test]
fn conventional_commit_add_type() {
	let options = conventional_commit_options();
	let mut types = options.types.clone();
	types.push(String::from("docs"));
	let mutated_options = RummitOptions { types, ..options };

	let commit_regex = CommitRegex::from(&mutated_options);

	assert_eq!(
		commit_regex.to_string(),
		r"^(revert: )?(feat|fix|docs)(\(.+\))?!?: .{1,52}$"
	)
}

#[test]
#[should_panic]
fn conventional_commit_no_type() {
	let options = conventional_commit_options();
	let mutated_options = RummitOptions {
		types: vec![],
		..options
	};

	let _commit_regex = CommitRegex::from(&mutated_options);
}

pub fn conventional_commit_options() -> RummitOptions {
	RummitOptions {
		enabled: true,
		debug: true,
		allow: AllowOptions {
			revert: true,
			breaking_change: true,
		},
		description_length: DescriptionLengthOptions { min: 1, max: 52 },
		types: vec![String::from("feat"), String::from("fix")],
		scope: ScopeOptions {
			required: false,
			_type: ScopeType::Flexible,
			values: None,
		},
	}
}
