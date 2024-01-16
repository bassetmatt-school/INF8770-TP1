use std::{
	collections::{hash_map, HashMap},
	fs,
};

fn init_dict(msg: &str) -> HashMap<String, String> {
	let mut dict = HashMap::new();
	let mut n_symb = 0;
	for c in msg.chars() {
		let c = c.to_string();
		// If the character is not in the dictionary, we add it
		if let hash_map::Entry::Vacant(e) = dict.entry(c) {
			let bin = format!("{n_symb:b}");
			e.insert(bin.clone());
			n_symb += 1;
		}
	}
	dict
}

fn update_dict_size(dict: &mut HashMap<String, String>) {
	let size = dict.len();
	let size_log = log_size(size);
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
	let chars: Vec<char> = msg.chars().collect();
	while i < chars.len() {
		let mut next = chars[i].to_string();
		let mut next_extra = chars[i].to_string();
		while dict.contains_key(next_extra.as_str()) && i < chars.len() {
			i += 1;
			next = next_extra.clone();
			if i < chars.len() {
				next_extra.push(chars[i]);
			}
		}
		let bin_code = dict.get(next.as_str()).unwrap().clone();
		length += bin_code.len();
		compressed.push_str(&bin_code);

		if i < chars.len() {
			let bin = format!("{n_symb:b}");
			dict.insert(next_extra, bin);
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

fn main() {
	const FILES: [&str; 5] = [
		"data/textes/texte_1.txt",
		"data/textes/texte_2.txt",
		"data/textes/texte_3.txt",
		"data/textes/texte_4.txt",
		"data/textes/texte_5.txt",
	];
	let msg = fs::read_to_string(FILES[0]).unwrap();
	let mut dict = init_dict(&msg);
	update_dict_size(&mut dict);
	println!("{:?}", dict);
	let (compressed, length) = compress(&msg, &mut dict);
	println!("Compressed: {}", compressed);
	println!("Length: {}", length);
	println!("Dict {:?}", dict);
	let mut keys = dict.keys().collect::<Vec<&String>>();
	keys.sort_by_key(|k| i32::from_str_radix(dict.get(*k).unwrap(), 2).unwrap());
	for k in keys {
		println!("{}: {}", k, dict.get(k).unwrap());
	}
}
