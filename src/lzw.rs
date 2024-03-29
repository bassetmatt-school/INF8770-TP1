use std::{
	collections::{hash_map, HashMap},
	fs,
	time::Instant,
};

use crate::utils::{log_size, stats, Unit};

fn init_dict(msg: &[u8]) -> HashMap<&[u8], String> {
	let mut dict = HashMap::new();
	let mut n_symb = 0;
	for i in 0..msg.len() {
		let c = &msg[i..i + 1];
		// If the character is not in the dictionary, we add it
		if let hash_map::Entry::Vacant(e) = dict.entry(c) {
			e.insert(format!("{n_symb:b}"));
			n_symb += 1;
		}
	}
	dict
}

fn update_dict_size(dict: &mut HashMap<&[u8], String>, one_zero: bool) {
	if one_zero {
		dict.iter_mut().for_each(|(_, v)| {
			v.insert(0, '0');
		});
	} else {
		let size_log = log_size(dict.len());
		// Avoids infinite loop
		if size_log == 0 {
			return;
		}
		dict.iter_mut().for_each(|(_, v)| {
			let new_zeros = size_log - v.len();
			for _ in 0..new_zeros {
				v.insert(0, '0');
			}
		});
	}
}

fn compress<'a>(msg: &'a [u8], dict: &mut HashMap<&'a [u8], String>) -> (String, usize) {
	// Compressed message
	let mut compressed = String::new();
	let mut n_symb = dict.len();
	let mut i = 0;
	let mut length = 0;
	// Main loop while the whole message is not compressed
	while i < msg.len() {
		// Beginning of the next symbol
		let init_i = i;
		// The next encoded symbol
		let mut next = &msg[i..i + 1];
		// The next new dictionary entry
		let mut next_extra = &msg[i..i + 1];

		// Checks to get the largest symbol in the dictionary
		while dict.contains_key(next_extra) && i < msg.len() {
			i += 1;
			next = next_extra;
			if i < msg.len() {
				next_extra = &msg[init_i..i + 1];
			}
		}
		// Gets the corresponding binary code and adds it to the message
		let bin_code = dict.get(next).unwrap().clone();
		length += bin_code.len();
		compressed.push_str(&bin_code);
		// Adds the new symbol to the dictionary
		if i < msg.len() {
			let bin = format!("{n_symb:b}");
			n_symb += 1;
			// Checks if the dictionary's size needs to be increased (padding zeroes)
			if log_size(n_symb) > bin_code.len() {
				update_dict_size(dict, true);
			}
			dict.insert(next_extra, bin);
		}
	}
	(compressed, length)
}

#[allow(dead_code)]
/// Prints the dictionary
fn print_dict(dict: &HashMap<&[u8], String>) {
	let mut keys = dict.keys().collect::<Vec<&&[u8]>>();
	keys.sort_by_key(|k| i32::from_str_radix(dict.get(*k).unwrap(), 2).unwrap());
	for k in keys {
		println!("{:?}: {}", k, dict.get(k).unwrap());
	}
}

/// Compresses a message using the LZW algorithm
pub fn run(msg: &[u8], write: bool, verbose: usize, unit: Option<Unit>) -> (f32, f32) {
	// Starting
	let start_time = Instant::now();
	// Creates the dictionary from the message's symbols
	let mut dict = init_dict(msg);
	// Avoid 0-sized message if the dictionary only contains 1 symbol
	let init_length = if log_size(dict.len()) == 0 {
		msg.len()
	} else {
		msg.len() * log_size(dict.len())
	};
	// Adds 0s to the dictionary's symbols to have the same length
	update_dict_size(&mut dict, false);
	let init_time = start_time.elapsed();

	// Compresses the message
	let start_compress = Instant::now();
	let (_compressed, length) = compress(msg, &mut dict);
	let compress_time = start_compress.elapsed();
	// Total time
	let total_time = start_time.elapsed();

	// Writes the compressed message to a file
	if write {
		fs::create_dir_all("../out").expect("Error creating output folder");
		fs::write("../out/rust_comp.bin", _compressed).expect("Failed to write file");
	}

	// Info
	if verbose > 0 {
		println!("Initial length: {}", init_length);
		println!("Compressed length: {}", length);
		println!(
			"Compression ratio: {:.2}%",
			100. - (length as f64) / (init_length as f64) * 100.
		);
		println!(
			"Compression factor: {:.2}",
			(init_length as f64) / (length as f64)
		);
		println!();
		let unit = unit.unwrap_or(Unit::Ms);
		println!(
			"Init time: {:.2} {unit}",
			init_time.as_secs_f32() * unit.factor()
		);
		println!(
			"Compress time: {:.2} {unit}",
			compress_time.as_secs_f32() * unit.factor()
		);
		println!(
			"Total time: {:.2} {unit}",
			total_time.as_secs_f32() * unit.factor()
		);
	}
	(init_time.as_secs_f32(), compress_time.as_secs_f32())
}

/// Runs the LZW algorithm on a message multiple times and prints the mean and standard deviation of the times
pub fn stats_run(msg: &[u8], n_run: usize, verbose: usize, unit: Option<Unit>) {
	// Different times for execution
	let mut init_times = Vec::new();
	let mut compress_times = Vec::new();
	let mut total_times = Vec::new();
	// Runs the algorithm multiple times
	for _ in 0..n_run {
		let (init_time, compress_time) = run(msg, false, 0, unit);
		init_times.push(init_time);
		compress_times.push(compress_time);
		total_times.push(init_time + compress_time);
	}
	// Stats on data
	let (mean_init, std_init) = stats(&init_times);
	let (mean_compress, std_compress) = stats(&compress_times);
	let (mean_total, std_total) = stats(&total_times);

	// Unit of time for display
	let unit = unit.unwrap_or(Unit::Ms);
	// Sub times
	if verbose > 1 {
		println!(
			"Init time: ({mean:.2} ± {err:.2}) {unit}",
			mean = mean_init * unit.factor(),
			err = std_init * unit.factor(),
		);
		println!(
			"Compress time: ({mean:.2} ± {err:.2}) {unit}",
			mean = mean_compress * unit.factor(),
			err = std_compress * unit.factor(),
		);
	}
	// Total time
	if verbose > 0 {
		println!(
			"Total time: ({mean:.2} ± {err:.2}) {unit}",
			mean = mean_total * unit.factor(),
			err = std_total * unit.factor(),
		);
	}
}
