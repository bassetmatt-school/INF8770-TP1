#![allow(dead_code)]
use std::collections::{hash_map, HashMap};

use crate::utils::{log_size, optimal_repeat_counter};
trait MessageFormat {}
impl MessageFormat for u8 {}
impl MessageFormat for u32 {}

struct CountedValue<T> {
	count: usize,
	value: T,
}

impl<T> CountedValue<T> {
	fn new(value: T) -> Self {
		Self { count: 1, value }
	}
}

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

fn optimal_bit_counter(msg: &[u32], dict: HashMap<u32, String>) -> usize {
	let mut map = HashMap::new();
	// Counting elements
	msg.iter().for_each(|&x| {
		*map.entry(x).or_insert(0) += 1;
	});
	let repeat = map.values().cloned().collect::<Vec<usize>>();
	optimal_repeat_counter(repeat.as_slice(), dict.len())
}
