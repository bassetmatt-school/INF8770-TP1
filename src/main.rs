mod lzw;
mod rle;
mod utils;
use std::env;

use image::{DynamicImage, ImageFormat};
use utils::Unit;

enum Method {
	Lzw,
	Rle,
}

fn main() {
	let args = env::args().collect::<Vec<String>>();
	let single_run = args.get(1).is_some_and(|x| x == "true");
	let method = match args.get(2) {
		Some(s) => match s.to_lowercase().as_str() {
			"lzw" => Method::Lzw,
			"rle" => Method::Rle,
			_ => panic!("Invalid method"),
		},
		None => Method::Lzw,
	};

	const MESSAGES: [&[u8]; 5] = [
		include_str!("../data/textes/texte_1.txt").as_bytes(),
		include_str!("../data/textes/texte_2.txt").as_bytes(),
		include_str!("../data/textes/texte_3.txt").as_bytes(),
		include_str!("../data/textes/texte_4.txt").as_bytes(),
		include_str!("../data/textes/texte_5.txt").as_bytes(),
	];

	let images: [DynamicImage; 5] = [
		load_img!("../data/images/image_1.png"),
		load_img!("../data/images/image_2.png"),
		load_img!("../data/images/image_3.png"),
		load_img!("../data/images/image_4.png"),
		load_img!("../data/images/image_5.png"),
	];

	// Text messages
	println!("===================================");
	println!("               Texts               ");
	println!("===================================");
	(0..MESSAGES.len()).for_each(|i| {
		println!("-----------------------------");
		println!("Text {}", i + 1);
		let image = false;
		let write = false;
		const N_TEXT_RUN: usize = 1000;
		match method {
			Method::Lzw => {
				if single_run {
					lzw::run(MESSAGES[i], write, 1, Some(Unit::Us));
				} else {
					lzw::stats_run(MESSAGES[i], N_TEXT_RUN, 1, Some(Unit::Us));
				}
			},
			Method::Rle => {
				if single_run {
					rle::run(MESSAGES[i], write, image, 1, Some(Unit::Us));
				} else {
					rle::stats_run(MESSAGES[i], N_TEXT_RUN, image, 1, Some(Unit::Us));
				}
			},
		}
		println!();
	});

	// Images
	println!("====================================");
	println!("               Images               ");
	println!("====================================");
	(0..images.len()).for_each(|i| {
		println!("-----------------------------");
		println!("Image {}", i + 1);
		let image = true;
		let write = false;
		const N_IMAGE_RUN: usize = 100;
		match method {
			Method::Lzw => {
				if single_run {
					lzw::run(images[i].as_bytes(), write, 1, Some(Unit::Ms));
				} else {
					lzw::stats_run(images[i].as_bytes(), N_IMAGE_RUN, 1, Some(Unit::Ms));
				}
			},
			Method::Rle => {
				if single_run {
					rle::run(images[i].as_bytes(), write, image, 1, Some(Unit::Ms));
				} else {
					rle::stats_run(images[i].as_bytes(), N_IMAGE_RUN, image, 1, Some(Unit::Ms));
				}
			},
		}
		println!();
	});
}
