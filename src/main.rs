use std::{
	collections::{hash_map, HashMap},
	time::Instant,
};

fn init_dict(msg: &str) -> HashMap<String, String> {
	let mut dict = HashMap::new();
	let mut n_symb = 0;
	for i in 0..msg.len() {
		let c = msg[i..i + 1].to_string();
		// If the character is not in the dictionary, we add it
		if let hash_map::Entry::Vacant(e) = dict.entry(c) {
			e.insert(format!("{n_symb:b}"));
			n_symb += 1;
		}
	}
	dict
}

fn update_dict_size(dict: &mut HashMap<String, String>) {
	let size_log = log_size(dict.len());
	dict.iter_mut().for_each(|(_, v)| {
		let v_int = v.parse::<usize>().unwrap();
		*v = format!("{v_int:0width$}", width = size_log);
	});
}

fn compress(msg: &str, dict: &mut HashMap<String, String>) -> (String, usize) {
	let mut compressed = String::new();
	let mut n_symb = dict.len();
	let mut i = 0;
	let mut length = 0;
	while i < msg.len() {
		let init_i = i;
		let mut next = &msg[i..i + 1];
		let mut next_extra = &msg[i..i + 1];
		while dict.contains_key(next_extra) && i < msg.len() {
			i += 1;
			next = next_extra;
			if i < msg.len() {
				next_extra = &msg[init_i..i + 1];
			}
		}
		let bin_code = dict.get(next).unwrap().clone();
		length += bin_code.len();
		compressed.push_str(&bin_code);

		if i < msg.len() {
			let bin = format!("{n_symb:b}");
			dict.insert(next_extra.to_string(), bin);
		}
		if log_size(n_symb) >= bin_code.len() {
			update_dict_size(dict);
		}
		n_symb = dict.len();
		// compressed.push_str(&String::from("'"));
	}
	(compressed, length)
}

fn log_size(x: usize) -> usize {
	(x as f64).log2().ceil() as usize
}

#[allow(dead_code)]
fn print_dict(dict: &HashMap<String, String>) {
	let mut keys = dict.keys().collect::<Vec<&String>>();
	keys.sort_by_key(|k| i32::from_str_radix(dict.get(*k).unwrap(), 2).unwrap());
	for k in keys {
		println!("{}: {}", k, dict.get(k).unwrap());
	}
}

fn main() {
	let start_time = Instant::now();
	const MESSAGES: [&str; 6] = [
		include_str!("../data/textes/texte_1.txt"),
		include_str!("../data/textes/texte_2.txt"),
		include_str!("../data/textes/texte_3.txt"),
		include_str!("../data/textes/texte_4.txt"),
		include_str!("../data/textes/texte_5.txt"),
		include_str!("../data/textes/texte_6.txt"),
	];
	let msg = MESSAGES[5];
	let mut dict = init_dict(msg);
	let init_length = msg.len() * (log_size(dict.len()) as usize);
	let loading_time = start_time.elapsed();
	update_dict_size(&mut dict);

	// println!("Dict: {:?}", dict);
	let start_compress = Instant::now();
	let (_compressed, length) = compress(msg, &mut dict);
	let compress_time = start_compress.elapsed();
	let total_time = start_time.elapsed();
	// println!("Compressed: {}", _compressed);
	println!("Length: {}", length);
	println!(
		"Compression ratio: {:.2}%",
		100. - (length as f64) / (init_length as f64) * 100.
	);
	println!(
		"Compression factor: {:.2}",
		(init_length as f64) / (length as f64)
	);
	println!("Load time: {:.2}ms", loading_time.as_secs_f32() * 1000.);
	println!(
		"Compress time: {:.2}ms",
		compress_time.as_secs_f32() * 1000.
	);
	println!("Total time: {:.2}ms", total_time.as_secs_f32() * 1000.);
	// print_dict(&dict);
}
