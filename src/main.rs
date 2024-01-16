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
	let msg = fs::read_to_string(FILES[4]).unwrap();
	let mut dict = init_dict(&msg);
	update_dict_size(&mut dict);
	println!("{:?}", dict);
}
