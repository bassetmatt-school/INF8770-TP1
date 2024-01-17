use std::fmt::Display;

#[derive(Debug, Copy, Clone)]
pub enum Unit {
	_S,
	Ms,
	Us,
}

impl Display for Unit {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Unit::_S => write!(f, "s"),
			Unit::Ms => write!(f, "ms"),
			Unit::Us => write!(f, "µs"),
		}
	}
}
impl Unit {
	pub fn factor(&self) -> f32 {
		match self {
			Unit::_S => 1.,
			Unit::Ms => 1e3,
			Unit::Us => 1e6,
		}
	}
}

/// Returns the mean and standard deviation of a vector
pub fn stats(v: &Vec<f32>) -> (f32, f32) {
	let mut mean = 0.;
	let mut variance = 0.;
	v.iter().for_each(|x| {
		mean += x;
		// Using the alternative formula for the variance
		variance += x.powi(2);
	});
	mean /= v.len() as f32;
	variance /= v.len() as f32;
	variance -= mean.powi(2);
	(mean, variance.sqrt())
}

#[macro_export]
macro_rules! load_img(
	($p:expr) => {
		image::load_from_memory_with_format(include_bytes!($p), ImageFormat::Png).unwrap()
	};
);
