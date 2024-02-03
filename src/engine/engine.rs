use crate::ui;

use crossterm::event::KeyCode;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use std::time::Duration;
use std::thread;

use super::tetromino::TetrominoType;

pub enum KeyPressed {
	ROTATE,
	LEFT,
	RIGHT,
	DOWN,
	NONE,
}

pub struct Game {
	board: [[TetrominoType; 10]; 20],
	delay: Duration,
	rx_key_event: UnboundedReceiver<KeyCode>,
}

impl Game {
	pub fn new(rx_key_event: UnboundedReceiver<KeyCode>) -> Self {
		let mut board = [[TetrominoType::None; 10]; 20];
		board[0][0] = TetrominoType::I;
		board[0][1] = TetrominoType::O;
		board[0][2] = TetrominoType::J;
		board[0][3] = TetrominoType::L;
		board[0][4] = TetrominoType::S;
		board[0][5] = TetrominoType::Z;
		board[0][6] = TetrominoType::T;
		let delay = Duration::from_millis(1000);
		Self {
			board,
			delay,
			rx_key_event,
		}
	}

	pub fn get_board(&self) -> [[TetrominoType; 10]; 20] {
		self.board
	}

	pub fn get_delay(&self) -> Duration {
		self.delay
	}

	fn key_press_listener(&mut self) -> KeyPressed {
        match self.rx_key_event.try_recv() {
            Ok(key) => {
				match key {
					KeyCode::Up => KeyPressed::ROTATE,
					KeyCode::Left => KeyPressed::LEFT,
					KeyCode::Right => KeyPressed::RIGHT,
					KeyCode::Down => KeyPressed::DOWN,
					_ => KeyPressed::NONE
				}
            }
            Err(_e) => KeyPressed::NONE,
        }
    }

	fn run(&mut self) -> Result<(), std::io::Error> {
		let delay = Duration::from_millis(1000);

        Ok(())
    }
}