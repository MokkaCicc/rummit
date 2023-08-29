use std::error::Error;

use super::options::RummitOptions;

pub trait OptionsLoader {
	fn load(&self) -> Result<RummitOptions, Box<dyn Error>>;
}
