use std::io::{self, Write};

use std::io::{stdout, Stdout};
use crossterm::event::KeyCode;

use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use crossterm::{cursor, terminal, ExecutableCommand};
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};

use crate::utils::terminal::colorize::{colorize_background, Colors};
use crate::engine::engine::Game;

#[derive(Debug)]
pub enum SnakeGameState {
    SnakeDied,
    Quit,
}

pub struct UI {
	stdout: Stdout,
	game: Game,
    rx_key_event: UnboundedReceiver<KeyCode>,
    tx_game_state: UnboundedSender<SnakeGameState>,
}


impl UI {
    pub fn new(
        rx_key_event: UnboundedReceiver<KeyCode>,
        tx_game_state: UnboundedSender<SnakeGameState>,
	) -> Result<Self, std::io::Error> {
        let mut stdout = stdout();
		
        enable_raw_mode()?;
		
        stdout
            .execute(terminal::Clear(terminal::ClearType::All))?
            .execute(cursor::Hide)?
            .execute(cursor::EnableBlinking)?;

		
		let game = Game::new();

        Ok(
			Self {
				stdout,
				game,
				rx_key_event,
				tx_game_state,
			}
		)
    }

	fn listen_for_key_press(&mut self) {
        match self.rx_key_event.try_recv() {
            Ok(key) => {
                match key {
					KeyCode::Left => self.game.move_left(),
					KeyCode::Right => self.game.move_right(),
					KeyCode::Down => {self.game.fall();},
					_ => return
                }
            }
            Err(_e) => return,
        }
    }

	pub async fn run(&mut self) -> Result<(), std::io::Error> {
		self.render_corners()?;
		let mut a: u8 = 0;
		loop {
			// if poll(Duration::from_millis(1)).unwrap() {
			// 	if let Event::Key(key_event) = read().unwrap() {
			// 		println!("key: {:?}, {:?}", key_event.code, key_event.kind);
			// 	}
			// }
			a = (a + 1) % 60;
			if a % 60 == 0 {
				self.game.update()?;
			}
			self.render_board()?;
			self.listen_for_key_press();
		
			io::stdout().flush().unwrap();

			tokio::time::sleep(self.game.get_delay()/60).await
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