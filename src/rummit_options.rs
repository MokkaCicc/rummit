use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RummitOptions {
	pub enabled: bool,
	pub debug: bool,
	pub allow: AllowOptions,
	pub description_length: DescriptionLengthOptions,
	pub types: Vec<String>,
	pub scope: ScopeOptions,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AllowOptions {
	pub revert: bool,
	pub breaking_change: bool,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DescriptionLengthOptions {
	pub min: i32,
	pub max: i32,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ScopeOptions {
	pub required: bool,
	#[serde(alias = "type")]
	pub _type: ScopeType,
	pub values: Option<Vec<String>>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum ScopeType {
	Value,
	ProjectItem,
	Flexible,
}
