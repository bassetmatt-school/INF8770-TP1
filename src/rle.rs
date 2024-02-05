#![allow(dead_code)]
use std::{
	collections::{hash_map, HashMap},
	fs,
	time::Instant,
};

use crate::utils::{log_size, optimal_repeat_counter, stats, Unit};

fn convert_message(msg: &[u8], image: bool) -> Vec<u32> {
	let mut new_msg = Vec::new();
	if image {
		msg.chunks_exact(4).for_each(|px| {
			// Codes the 4 u8 of the rgba on a u32
			let rgba = u32::from_ne_bytes(px.try_into().unwrap());
			new_msg.push(rgba);
		})
	} else {
		msg.iter().for_each(|&x| new_msg.push(x as u32))
	}
	new_msg
}

fn init_dict(msg: &[u32]) -> HashMap<u32, String> {
	let mut dict = HashMap::new();
	let mut n_symb = 0;
	msg.iter().for_each(|&char| {
		if let hash_map::Entry::Vacant(e) = dict.entry(char) {
			e.insert(format!("{n_symb:b}"));
			n_symb += 1;
		}
	});
	dict
}

fn update_dict_size(dict: &mut HashMap<u32, String>) {
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

fn repeat_values_count(array: &[u32]) -> Vec<(usize, u32)> {
	let mut repeat_values = Vec::new();
	// Counting elements
	let mut x_prec: u32 = array[0];
	let mut n_rep: usize = 0;
	array.iter().for_each(|&x| {
		if x_prec != x {
			repeat_values.push((n_rep, x_prec));
			n_rep = 1;
			x_prec = x;
		} else {
			n_rep += 1;
		};
	});
	// Don't forget the last repetition
	repeat_values.push((n_rep, x_prec));
	repeat_values
}

fn compress(msg: &[u32], dict: &mut HashMap<u32, String>) -> (String, usize) {
	let mut compressed = String::new();
	let mut length = 0;

	let repeat_values = repeat_values_count(msg);
	// Computes the optimal number of bits for repetition counter
	let k_bits = {
		let repeat = repeat_values
			.iter()
			.map(|(r, _)| *r)
			.collect::<Vec<usize>>();
		optimal_repeat_counter(&repeat, dict.len())
	};
	// Constructs the compressed message
	repeat_values.iter().for_each(|(count, value)| {
		// Adds the compressed value to the compressed message
		let bin_code = dict[value].clone();
		let out_str = format!("{:0width$b}{bin_code}", count, width = k_bits);
		compressed.push_str(&out_str);
		length += k_bits + dict[value].len();
	});
	(compressed, length)
}

pub fn run(msg: &[u8], write: bool, image: bool, verbose: usize, unit: Option<Unit>) -> (f32, f32) {
	// Starting
	let start_time = Instant::now();
	// Converts message to u32 format (for image and text)
	let msg = convert_message(msg, image);
	// Initializes the dictionary and fills zeroes
	let mut dict = init_dict(&msg);
	let init_length = if log_size(dict.len()) == 0 {
		msg.len()
	} else {
		msg.len() * log_size(dict.len())
	};
	update_dict_size(&mut dict);

	let init_time = start_time.elapsed();

	let start_compress = Instant::now();
	// Gets the repetitions of the message's values

	let (_compressed, length) = compress(&msg, &mut dict);

	let compress_time = start_compress.elapsed();
	let total_time = start_time.elapsed();

	// Writes the compressed message to a file
	if write {
		fs::create_dir_all("../out").expect("Error creating output folder");
		fs::write("../out/rle_comp.bin", _compressed).expect("Failed to write file");
	}

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

pub fn stats_run(msg: &[u8], n_run: usize, image: bool, verbose: usize, unit: Option<Unit>) {
	// Different times for execution
	let mut init_times = Vec::new();
	let mut compress_times = Vec::new();
	let mut total_times = Vec::new();
	// Runs the algorithm multiple times
	for _ in 0..n_run {
		let (init_time, compress_time) = run(msg, false, image, 0, unit);
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
