use rand::prelude::*;

use crate::engine::{self, engine::Game};

pub fn main() {
	let mut rng = rand::thread_rng();
	let y: f64 = rng.gen();
	let game = Game::new();

	for it in game.get_board() {
		for it2 in it {
			print!("\x1b[1;{}m{}\x1b[0m", rand::thread_rng().gen_range(41..48), "  ");
		}
		println!();
	}


	// for i in 0..110 {
	// 	println!("{i}: \x1b[{i}m{} \x1b[0m", "TEST");
	// }


    // Print bold yellow text on blue background
    println!("\x1b[1;33;44mBold Yellow Text on Blue Background\x1b[0m");
} 