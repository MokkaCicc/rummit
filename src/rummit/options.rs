use std::error::Error;

pub struct RummitOptions {
	pub enabled: bool,
	pub debug: bool,
	pub allow: AllowOptions,
	pub description_length: DescriptionLengthOptions,
	pub types: Vec<String>,
	pub scope: ScopeOptions,
}
impl RummitOptions {
	pub fn is_valid(&self) -> Result<(), Box<dyn Error>> {
		if self.description_length.min > self.description_length.max {
			return Err("Min description length is greater than max description length.".into());
		}

		if self.types.is_empty() {
			return Err("Should have at least one type.".into());
		}

		if self.scope._type == ScopeType::Value {
			match self.scope.values.clone() {
				Some(values) => {
					if values.is_empty() {
						return Err("With `scope.type` set to `Value`, `scope.values` need to have at least one value.".into());
					}
				}
				None => {
					return Err(
						"With `scope.type` set to `Value`, `scope.values` need to be created"
							.into(),
					)
				}
			}
		}

		Ok(())
	}
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

#[derive(PartialEq)]
pub enum ScopeType {
	Value,
	ProjectItem,
	Flexible,
}
