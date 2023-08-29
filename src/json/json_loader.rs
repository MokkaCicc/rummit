use std::error::Error;
use std::fs::File;
use std::io::BufReader;

use crate::rummit::options::RummitOptions;
use crate::rummit::options_loader::OptionsLoader;

use super::options_entity::RummitOptionsEntity;

const JSON_PATH: &str = "rummit.json";

pub struct JsonLoader;

impl JsonLoader {
	pub fn new() -> Self {
		Self
	}
}

impl OptionsLoader for JsonLoader {
	fn load(&self) -> Result<RummitOptions, Box<dyn Error>> {
		let file = File::open(JSON_PATH)?;
		let reader = BufReader::new(file);
		let options_json: RummitOptionsEntity = serde_json::from_reader(reader)?;
		Ok(options_json.into())
	}
}
