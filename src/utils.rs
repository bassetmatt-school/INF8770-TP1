use std::{cmp::Ordering, fmt::Display};

/// Returns the number of bits needed to encode a number
pub fn log_size(x: usize) -> usize {
	(x as f64).log2().ceil() as usize
}

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

fn _argmin<T: PartialOrd>(v: &Vec<T>) -> usize {
	let mut argmin = 0;
	for i in 1..v.len() {
		if v[i] < v[argmin] {
			argmin = i;
		}
	}
	argmin
}

fn _optimal_repeat_counter(repeat: &[usize], dict_size: u32) -> usize {
	// Maximum number of bits needed to encode a repeat
	let max_k = log_size(*repeat.iter().max().unwrap());
	let symb_size = log_size(dict_size as usize);
	let mut waste_list = Vec::new();
	// Test for each number of bits
	for k in 1..=max_k {
		// Bits wasted for non optimal encoding
		let mut waste = 0;
		for &r in repeat.iter() {
			// Number of bits needed to optimally encode the repeat
			let opti_bits = log_size(r);
			waste += match opti_bits.cmp(&k) {
				// k too big, wasted = difference
				Ordering::Less => k - opti_bits,
				// Optimal case
				Ordering::Equal => 0,
				// k too small
				Ordering::Greater => {
					// We use (x/ 2^k) + 1 counters, each of size k + symb_size
					let used = ((r >> k) + 1) * (k + symb_size);
					// Ideally, we would use opti_bits + symb_size
					let ideal = opti_bits + symb_size;
					// Difference is wasted
					ideal - used
				},
			}
		}
		waste_list.push(waste);
	}
	// Return the k value that minimizes the waste
	_argmin(&waste_list)
}
