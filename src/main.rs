mod lzw;
mod rle;
mod utils;
use image::{DynamicImage, ImageFormat};
use utils::Unit;

fn main() {
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

	const SINGLE_RUN: bool = true;
	const USE_RLE: bool = true;
	const USE_LZW: bool = false;

	// Text messages
	println!("===================================");
	println!("               Texts               ");
	println!("===================================");
	(0..MESSAGES.len()).for_each(|i| {
		println!("-----------------------------");
		println!("Text {}", i + 1);
		if USE_LZW {
			if SINGLE_RUN {
				lzw::run(MESSAGES[i], false, 1, Some(Unit::Us));
			} else {
				lzw::stats_run(MESSAGES[i], 1000, 1, Some(Unit::Us));
			}
		}
		if USE_RLE {
			if SINGLE_RUN {
				rle::run(MESSAGES[i], false, false, 1, Some(Unit::Us));
			} else {
				rle::stats_run(MESSAGES[i], 1000, false, 1, Some(Unit::Us));
			}
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
		if USE_LZW {
			if SINGLE_RUN {
				lzw::run(images[i].as_bytes(), false, 1, Some(Unit::Ms));
			} else {
				lzw::stats_run(images[i].as_bytes(), 100, 1, Some(Unit::Ms));
			}
		}
		if USE_RLE {
			if SINGLE_RUN {
				rle::run(images[i].as_bytes(), true, false, 1, Some(Unit::Ms));
			} else {
				rle::stats_run(images[i].as_bytes(), 100, true, 1, Some(Unit::Ms));
			}
		}
		println!();
	});
}
