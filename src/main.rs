mod lzw;
mod rle;
mod utils;
use image::{DynamicImage, GenericImageView, ImageFormat, Pixel};
use utils::Unit;

fn main() {
	const MESSAGES: [&[u8]; 6] = [
		include_str!("../data/textes/texte_1.txt").as_bytes(),
		include_str!("../data/textes/texte_2.txt").as_bytes(),
		include_str!("../data/textes/texte_3.txt").as_bytes(),
		include_str!("../data/textes/texte_4.txt").as_bytes(),
		include_str!("../data/textes/texte_5.txt").as_bytes(),
		include_str!("../data/textes/texte_6.txt").as_bytes(),
	];

	let images: [DynamicImage; 5] = [
		load_img!("../data/images/image_1.png"),
		load_img!("../data/images/image_2.png"),
		load_img!("../data/images/image_3.png"),
		load_img!("../data/images/image_4.png"),
		load_img!("../data/images/image_5.png"),
	];

	const SINGLE_RUN: bool = true;

	// Text messages
	println!("===================================");
	println!("               Texts               ");
	println!("===================================");
	(0..(MESSAGES.len() - 1)).for_each(|i| {
		println!("-----------------------------");
		println!("Text {}", i + 1);
		if SINGLE_RUN {
			lzw::run(MESSAGES[i], false, 1, Some(Unit::Us));
		} else {
			lzw::stats_run(MESSAGES[i], 100, 1, Some(Unit::Us));
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
		if SINGLE_RUN {
			lzw::run(images[i].as_bytes(), false, 1, Some(Unit::Ms));
		} else {
			lzw::stats_run(images[i].as_bytes(), 50, 1, Some(Unit::Ms));
		}
		println!();
	});
	fn n_channels(image: &DynamicImage) -> usize {
		image.pixels().next().unwrap().2.channels().len()
	}
	(0..images.len()).for_each(|i| {
		println!(
			"Pixel: {:?}. Channels {:?}",
			images[i].pixels().nth(324).unwrap(),
			n_channels(&images[i]),
		);
	});
}
