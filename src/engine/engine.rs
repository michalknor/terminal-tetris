use crossterm::event::KeyCode;
use rand::rngs::StdRng;
use rand::SeedableRng;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use std::{thread::current, time::Duration};

use super::tetromino::{Tetromino, TetrominoType};


pub const WIDTH: u8 = 10;
pub const HEIGHT: u8 = 20;


pub enum KeyPressed {
	ROTATE,
	LEFT,
	RIGHT,
	DOWN,
	NONE,
}

pub struct Game {
	board_without_current: [[TetrominoType; 10]; 20],
	board_with_current: [[TetrominoType; 10]; 20],
	delay: Duration,
	rng: StdRng,
	bag: Vec<Tetromino>,
	current_tetronimo: Tetromino,
}

impl Game {
	pub fn new() -> Self {
		let board = [[TetrominoType::None; 10]; 20];
		let delay = Duration::from_millis(1000);
        let mut rng = StdRng::seed_from_u64(1);
		let mut bag = Tetromino::new_bag(&mut rng);
		let current_tetronimo = bag.pop().unwrap();

		Self {
			board_without_current: board,
			board_with_current: board,
			delay,
			rng,
			bag,
			current_tetronimo,
		}
	}

	pub fn get_board(&self) -> [[TetrominoType; 10]; 20] {
		self.board_with_current
	}

	pub fn get_delay(&self) -> Duration {
		self.delay
	}

	pub fn update(&mut self) -> Result<(), std::io::Error> {
		if self.can_fall() {
			self.fall();
			return Ok(())
		}
		
		self.place_current_tetronimo();
		self.draw_from_bag();

		Ok(())
	}

	pub fn move_left(&mut self) {
		if !self.can_move_left() {
			return;
		}
		self.current_tetronimo.move_left();
		self.update_board();
	}

	pub fn move_right(&mut self) {
		if !self.can_move_right() {
			return;
		}
		self.current_tetronimo.move_right();
		self.update_board();
	}

	fn can_move_left(&mut self) -> bool {
		for (x, y) in self.current_tetronimo.get_blocks() {
			if y >= 20 {
				continue;
			}
			if x == 0 {
				return false;
			}
			if self.board_without_current[y][x-1] != TetrominoType::None {
				return false;
			}
		}

		true
	}

	fn can_move_right(&mut self) -> bool {
		for (x, y) in self.current_tetronimo.get_blocks() {
			if y >= 20 {
				continue;
			}
			if x == 10-1 {
				return false;
			}
			if self.board_without_current[y][x+1] != TetrominoType::None {
				return false;
			}
		}

		true
	}

	fn can_fall(&self) -> bool {
		for (x, y) in self.current_tetronimo.get_blocks() {
			if y >= 20 {
				continue;
			}
			if y <= 0 {
				return false;
			}
			if self.board_without_current[y-1][x] != TetrominoType::None {
				return false;
			}
		}

		true
	}

	fn fall(&mut self) {
		self.current_tetronimo.fall();
		self.update_board();
	}

	fn update_board(&mut self) {
		self.board_with_current = self.board_without_current.clone();

		for (x, y) in self.current_tetronimo.get_blocks() {
			if y >= 20 {
				continue;
			}
			self.board_with_current[y][x] = self.current_tetronimo.get_tetromino_type();
		}
	}

	fn place_current_tetronimo(&mut self) {
		self.board_without_current = self.board_with_current.clone();
	}

	fn draw_from_bag(&mut self) {
		self.current_tetronimo = self.bag.pop().unwrap();
		if self.bag.len() == 0 {
			self.bag = Tetromino::new_bag(&mut self.rng);
		}
		self.fall();
	}

	// fn key_press_listener(&mut self) -> KeyPressed {
    //     match self.rx_key_event.try_recv() {
    //         Ok(key) => {
	// 			match key {
	// 				KeyCode::Up => KeyPressed::ROTATE,
	// 				KeyCode::Left => KeyPressed::LEFT,
	// 				KeyCode::Right => KeyPressed::RIGHT,
	// 				KeyCode::Down => KeyPressed::DOWN,
	// 				_ => KeyPressed::NONE
	// 			}
    //         }
    //         Err(_e) => KeyPressed::NONE,
    //     }
    // }
}