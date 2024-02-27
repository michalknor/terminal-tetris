use std::io::{self, Write};

use std::io::{stdout, Stdout};
use crossterm::event::KeyCode;

use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use crossterm::{cursor, terminal, ExecutableCommand};
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};

use crate::engine::tetromino::TetrominoType;
use crate::utils::terminal::colorize::{colorize_background, colorize_text, Color};
use crate::engine::engine::{Game, KeyPressed};

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

	fn listen_for_key_press(&mut self) -> KeyPressed {
        match self.rx_key_event.try_recv() {
            Ok(key) => {
                match key {
					KeyCode::Left => KeyPressed::LEFT,
					KeyCode::Right => KeyPressed::RIGHT,
					KeyCode::Down => KeyPressed::DOWN,
					KeyCode::Char('a') => KeyPressed::ROTATE_LEFT,
					KeyCode::Char('d') => KeyPressed::ROTATE_RIGHT,
					_ => KeyPressed::NONE
                }
            }
            Err(_e) => KeyPressed::NONE,
        }
    }

	pub async fn run(&mut self) -> Result<(), std::io::Error> {
		// self.render_background()?;
		self.render_logo()?;
		self.render_corners()?;
		self.render_borders_for_next_tetronimo()?;
		loop {
			let key_pressed: KeyPressed = self.listen_for_key_press();
			self.game.update(key_pressed)?;
			
			self.render_next_tetronimo()?;
			self.render_board()?;
		
			io::stdout().flush().unwrap();

			tokio::time::sleep(self.game.get_delay()/60).await
		}
	}

	// fn render_background(&mut self) -> Result<(), std::io::Error> {
	// 	let horizontal_edge_line: String = "  ".repeat(20);
	// 	for i in 0..20+6 {
	// 		self.stdout
	// 			.execute(crossterm::cursor::MoveTo(0, i))?;
	// 		print!("{}", colorize_background(&horizontal_edge_line.to_string(), Color::GRAY))
	// 	}

	// 	Ok(())
	// }

	fn render_logo(&mut self) -> Result<(), std::io::Error> {
		self.stdout
			.execute(crossterm::cursor::MoveTo(2*6, 1))?;
		print!("{}", colorize_text(&"T".to_string(), TetrominoType::S.get_color()));
		print!("{}", colorize_text(&"E".to_string(), TetrominoType::L.get_color()));
		print!("{}", colorize_text(&"T".to_string(), TetrominoType::O.get_color()));
		print!("{}", colorize_text(&"R".to_string(), TetrominoType::Z.get_color()));
		print!("{}", colorize_text(&"I".to_string(), TetrominoType::I.get_color()));
		print!("{}", colorize_text(&"S".to_string(), TetrominoType::T.get_color()));

		Ok(())
	}

	fn render_board(&mut self) -> Result<(), std::io::Error> {
		for (i, it) in self.game.get_board().iter().enumerate() {
			self.stdout
				.execute(crossterm::cursor::MoveTo(5, (20-i+3) as u16))?;
			for it2 in it {
				print!("{}", colorize_background(&"  ".to_string(), it2.get_color()));
			}
		}

		Ok(())
	}

	fn render_borders_for_next_tetronimo(&mut self) -> Result<(), std::io::Error> {
		let horizontal_edge_line: String = "  ".repeat(4+2);
		
		self.stdout
			.execute(crossterm::cursor::MoveTo(3+20+2+2+2, (20-14+3-4) as u16))?;
		print!("{}", colorize_background(&horizontal_edge_line, Color::WHITE));

		for i in 0..4 {
			self.stdout
				.execute(crossterm::cursor::MoveTo(3+20+2+2+2, (20-14-i+3) as u16))?;
			print!("{}", colorize_background(&"  ".to_string(), Color::WHITE));

			self.stdout
				.execute(crossterm::cursor::MoveTo(3+20+2+2+2+2*5, (20-14-i+3) as u16))?;
			print!("{}", colorize_background(&"  ".to_string(), Color::WHITE));
		}
		
		self.stdout
			.execute(crossterm::cursor::MoveTo(3+20+2+2+2, (20-14+3+1) as u16))?;
		print!("{}", colorize_background(&horizontal_edge_line, Color::WHITE));

		Ok(())
	}

	fn render_next_tetronimo(&mut self) -> Result<(), std::io::Error> {
		let next_tetronimo = self.game.get_next_tetronimo();
		let next_tetronimo_block = colorize_background(&"  ".to_string(), next_tetronimo.get_tetromino_type().get_color());
		let next_tetronimo_blocks = next_tetronimo.get_blocks();


		for i in 0..4 {
			self.stdout
				.execute(crossterm::cursor::MoveTo(3+20+2+2+2+2, (20-14+3-4+1+i) as u16))?;
			for j in 0..4 {
				if next_tetronimo_blocks.contains(&(j+3, -i+22)) {
					print!("{}", next_tetronimo_block);
				}
				else {
					print!("  ");
				}
			}
		}

		Ok(())
	}

	fn render_corners(&mut self) -> Result<(), std::io::Error> {
		let horizontal_edge_line: String = "  ".repeat(10+2);
		
		self.stdout
			.execute(crossterm::cursor::MoveTo(3, 3_u16))?;
		print!("{}", colorize_background(&horizontal_edge_line, Color::WHITE));

		for i in 0..20 {
			self.stdout
				.execute(crossterm::cursor::MoveTo(3, (20-i+3) as u16))?;
			print!("{}", colorize_background(&"  ".to_string(), Color::WHITE));

			self.stdout
				.execute(crossterm::cursor::MoveTo(3+2+10*2, (20-i+3) as u16))?;
			print!("{}", colorize_background(&"  ".to_string(), Color::WHITE));
		}
		
		self.stdout
			.execute(crossterm::cursor::MoveTo(3, 3+20+1_u16))?;
		print!("{}", colorize_background(&horizontal_edge_line, Color::WHITE));

		Ok(())
	}
}