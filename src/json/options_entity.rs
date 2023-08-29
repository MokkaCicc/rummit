use serde::Deserialize;

use crate::rummit::options::*;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RummitOptionsEntity {
	pub enabled: bool,
	pub debug: bool,
	pub allow: AllowOptionsEntity,
	pub description_length: DescriptionLengthOptionsEntity,
	pub types: Vec<String>,
	pub scope: ScopeOptionsEntity,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AllowOptionsEntity {
	pub revert: bool,
	pub breaking_change: bool,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DescriptionLengthOptionsEntity {
	pub min: i32,
	pub max: i32,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScopeOptionsEntity {
	pub required: bool,
	#[serde(alias = "type")]
	pub _type: ScopeTypeEntity,
	pub values: Option<Vec<String>>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ScopeTypeEntity {
	Value,
	ProjectItem,
	Flexible,
}

impl Into<RummitOptions> for RummitOptionsEntity {
	fn into(self) -> RummitOptions {
		RummitOptions {
			enabled: self.enabled,
			debug: self.debug,
			allow: AllowOptions {
				revert: self.allow.revert,
				breaking_change: self.allow.breaking_change,
			},
			description_length: DescriptionLengthOptions {
				min: self.description_length.min,
				max: self.description_length.max,
			},
			types: self.types,
			scope: ScopeOptions {
				required: self.scope.required,
				_type: self.scope._type.into(),
				values: self.scope.values,
			},
		}
	}
}

impl Into<ScopeType> for ScopeTypeEntity {
	fn into(self) -> ScopeType {
		match self {
			ScopeTypeEntity::Value => ScopeType::Value,
			ScopeTypeEntity::ProjectItem => ScopeType::ProjectItem,
			ScopeTypeEntity::Flexible => ScopeType::Flexible,
		}
	}
}
