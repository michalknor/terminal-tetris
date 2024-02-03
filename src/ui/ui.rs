use rand::prelude::*;
use std::io::{self, Write};

use std::{
    borrow::Cow,
    io::{stdout, Stdout},
};

use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use crossterm::{cursor, style::Print, terminal, ExecutableCommand};
use tokio::sync::mpsc::unbounded_channel;
use crate::{ui::tetromino};



use crate::engine::engine::Game;

pub struct UI {
	stdout: Stdout,
	game: Game,
}


impl UI {
    pub fn new() -> Result<Self, std::io::Error> {
        let mut stdout = stdout();
		
        stdout
            .execute(terminal::Clear(terminal::ClearType::All))?
            .execute(cursor::Hide)?
            .execute(cursor::EnableBlinking)?;
		
        enable_raw_mode()?;

		let (tx_key_event, rx_key_event) = unbounded_channel();

		let game = Game::new(rx_key_event);

        Ok(Self {stdout, game})
    }

	pub async fn run(&mut self) -> Result<(), std::io::Error> {
		loop {
			self.run_game()?;
			tokio::time::sleep(self.game.get_delay()).await
		}
	}

	fn run_game(&mut self) -> Result<(), std::io::Error> {
		for (i, it) in self.game.get_board().iter().enumerate() {
			self.stdout
				.execute(crossterm::cursor::MoveTo(0, i as u16))?;
			for it2 in it {
				// print!("\x1b[1;{}m  \x1b[0m", rand::thread_rng().gen_range(41..48));
				print!("{}", it2.get_text());
			}
		}
		
		io::stdout().flush().unwrap();

		Ok(())
	}
}