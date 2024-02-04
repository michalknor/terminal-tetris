use std::io::{self, Write};

use std::{
    borrow::Cow,
    io::{stdout, Stdout},
};

use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use crossterm::{cursor, terminal, ExecutableCommand};
use tokio::sync::mpsc::unbounded_channel;

use crate::utils::terminal::colorize::{colorize_background, Colors};
use crate::engine::engine::Game;

pub struct UI {
	stdout: Stdout,
	game: Game,
}


impl UI {
    pub fn new() -> Result<Self, std::io::Error> {
        let mut stdout = stdout();
		
        enable_raw_mode()?;
		
        stdout
            .execute(terminal::Clear(terminal::ClearType::All))?
            .execute(cursor::Hide)?
            .execute(cursor::EnableBlinking)?;

		let (tx_key_event, rx_key_event) = unbounded_channel();

		let game = Game::new(rx_key_event);

        Ok(Self {stdout, game})
    }

	pub async fn run(&mut self) -> Result<(), std::io::Error> {
		self.render_corners()?;
		loop {
			self.game.update()?;
			self.render_board()?;
		
			io::stdout().flush().unwrap();

			tokio::time::sleep(self.game.get_delay()).await
		}

		// Ok(())
	}

	fn render_board(&mut self) -> Result<(), std::io::Error> {
		for (i, it) in self.game.get_board().iter().enumerate() {
			self.stdout
				.execute(crossterm::cursor::MoveTo(5, (20-i+5) as u16))?;
			for it2 in it {
				print!("{}", it2.get_text());
			}
		}

		Ok(())
	}

	fn render_corners(&mut self) -> Result<(), std::io::Error> {
		let horizontal_edge_line: String = "  ".repeat(10+2);
		
		self.stdout
			.execute(crossterm::cursor::MoveTo(3, 5 as u16))?;
		print!("{}", colorize_background(&horizontal_edge_line, Colors::WHITE));

		for i in 0..20 {
			self.stdout
				.execute(crossterm::cursor::MoveTo(3, (20-i+5) as u16))?;
			print!("{}", colorize_background(&"  ".to_string(), Colors::WHITE));

			self.stdout
				.execute(crossterm::cursor::MoveTo(3+2+10*2, (20-i+5) as u16))?;
			print!("{}", colorize_background(&"  ".to_string(), Colors::WHITE));
		}
		
		self.stdout
			.execute(crossterm::cursor::MoveTo(3, 5+20+1 as u16))?;
		print!("{}", colorize_background(&horizontal_edge_line, Colors::WHITE));

		Ok(())
	}
}