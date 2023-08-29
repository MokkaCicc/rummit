pub struct RummitOptions {
	pub enabled: bool,
	pub debug: bool,
	pub allow: AllowOptions,
	pub description_length: DescriptionLengthOptions,
	pub types: Vec<String>,
	pub scope: ScopeOptions,
}

pub struct AllowOptions {
	pub revert: bool,
	pub breaking_change: bool,
}

pub struct DescriptionLengthOptions {
	pub min: i32,
	pub max: i32,
}

pub struct ScopeOptions {
	pub required: bool,
	pub _type: ScopeType,
	pub values: Option<Vec<String>>,
}

pub enum ScopeType {
	Value,
	ProjectItem,
	Flexible,
}
