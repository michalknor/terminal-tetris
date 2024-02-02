use rand::prelude::*;
use std::io::{self, Write};
use std::time::Duration;
use std::thread;

use std::{
    borrow::Cow,
    io::{stdout, Stdout},
};

use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use crossterm::{cursor, style::Print, terminal, ExecutableCommand};



use crate::engine::{self, engine::Game};

pub fn main() -> Result<(), std::io::Error> {
	let mut rng = rand::thread_rng();
	let y: f64 = rng.gen();
	let game = Game::new();
	enable_raw_mode()?;

	let mut stdout = stdout();

	stdout
            .execute(terminal::Clear(terminal::ClearType::All))?
            .execute(cursor::Hide)?
            .execute(cursor::EnableBlinking)?;

	loop {
		stdout
		.execute(crossterm::cursor::MoveTo(0, 0))
		.unwrap();
		for (i, it) in game.get_board().iter().enumerate() {
			stdout.execute(crossterm::cursor::MoveTo(0, i as u16));
			for it2 in it {
				print!("\x1b[1;{}m{}\x1b[0m", rand::thread_rng().gen_range(41..48), "  ");
			}
		}
		// println!("\x1b[1;{}m{}\x1b[0m", rand::thread_rng().gen_range(41..48), "  ");
		thread::sleep(Duration::from_secs(1));
		// clear_previous_line();
	}

	// for i in 0..110 {
	// 	println!("{i}: \x1b[{i}m{} \x1b[0m", "TEST");
	// }
}

fn clear_previous_line() {
    for _ in 0..20 {
        print!("\x1B[2K\x1B[1A"); // Clear line and move cursor up
    }
    io::stdout().flush().unwrap();
}