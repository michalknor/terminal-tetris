use rand::rngs::StdRng;
use rand::SeedableRng;
use std::time::Duration;

use super::tetromino::{Tetromino, TetrominoType};


pub const WIDTH: u8 = 10;
pub const HEIGHT: u8 = 20;


pub enum KeyPressed {
	LEFT,
	RIGHT,
	DOWN,
	ROTATE_LEFT,
	ROTATE_RIGHT,
	NONE,
}

pub struct Game {
	board_without_current: [[TetrominoType; 10]; 20],
	board_with_current: [[TetrominoType; 10]; 20],
	delay: Duration,
	rng: StdRng,
	bag: Vec<Tetromino>,
	current_tetronimo: Tetromino,
	fall_speed: f64,
	fall_progress: f64
}

impl Game {
	pub fn new() -> Self {
		let board = [[TetrominoType::None; 10]; 20];
		let delay = Duration::from_millis(1000);
        let mut rng = StdRng::seed_from_u64(1);
		let mut bag = Tetromino::new_bag(&mut rng);
		let current_tetronimo = bag.pop().unwrap();
		let fall_speed: f64 = 0.01667;
		let fall_progress: f64 = 1.0;

		Self {
			board_without_current: board,
			board_with_current: board,
			delay,
			rng,
			bag,
			current_tetronimo,
			fall_speed,
			fall_progress,
		}
	}

	pub fn get_board(&self) -> [[TetrominoType; 10]; 20] {
		self.board_with_current
	}

	pub fn get_delay(&self) -> Duration {
		self.delay
	}

	pub fn update(&mut self, key_pressed: KeyPressed) -> Result<(), std::io::Error> {
		self.fall_progress += self.fall_speed;

		match key_pressed {
			KeyPressed::LEFT => self.move_left(),
			KeyPressed::RIGHT => self.move_right(),
			KeyPressed::DOWN => {
				if !self.fall() {
					self.fall_progress = 1.0;
				}
			},
			KeyPressed::ROTATE_LEFT => self.rotate_left(),
			KeyPressed::ROTATE_RIGHT => self.rotate_right(),
			KeyPressed::NONE => {},
		}

		if self.fall_progress < 1.0 {
			return Ok(())
		}

		self.fall_progress = 0.0;
		
		if self.fall() {
			return Ok(())
		}

		self.clear_rows();
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
			if x == 0 {
				return false;
			}
			if y >= 20 {
				continue;
			}
			if self.board_without_current[y as usize][x as usize - 1] != TetrominoType::None {
				return false;
			}
		}

		true
	}

	fn can_move_right(&mut self) -> bool {
		for (x, y) in self.current_tetronimo.get_blocks() {
			if x == 10-1 {
				return false;
			}
			if y >= 20 {
				continue;
			}
			if self.board_without_current[y as usize][x as usize + 1] != TetrominoType::None {
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
			if self.board_without_current[y as usize - 1][x as usize] != TetrominoType::None {
				return false;
			}
		}

		true
	}

	pub fn fall(&mut self) -> bool {
		if !self.can_fall() {
			return false;
		}
		self.current_tetronimo.fall();
		self.update_board();
		true
	}

	fn update_board(&mut self) {
		self.board_with_current = self.board_without_current.clone();

		for (x, y) in self.current_tetronimo.get_blocks() {
			if y >= 20 {
				continue;
			}
			self.board_with_current[y as usize][x as usize] = self.current_tetronimo.get_tetromino_type();
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

	fn rotate_left(&mut self) {
		self.current_tetronimo.rotate_left();
		if self.is_current_tetronimo_valid() {
			self.update_board();
			return;
		}
		self.current_tetronimo.rotate_right();
	}

	fn rotate_right(&mut self) {
		self.current_tetronimo.rotate_right();
		if self.is_current_tetronimo_valid() {
			self.update_board();
			return;
		}
		self.current_tetronimo.rotate_left();
	}

	fn is_current_tetronimo_valid(&self) -> bool {
		for (x, y) in self.current_tetronimo.get_blocks() {
			if y >= 20 {
				continue;
			}
			if y < 0 {
				return false;
			}
			if x >= 10 {
				return false;
			}
			if x < 0 {
				return false;
			}
			if self.board_without_current[y as usize][x as usize] != TetrominoType::None {
				return false;
			}
		}

		true
	}

	pub fn get_next_tetronimo(&mut self) -> &Tetromino {
		&self.bag[self.bag.len() - 1]
	}

	fn clear_rows(&mut self) {
		let mut cleared_rows: Vec<usize> = Vec::new();

		'loop_rows: for (i, row) in self.board_with_current.iter().enumerate() {
			for block in row {
				if *block == TetrominoType::None {
					continue 'loop_rows;
				}
			}
			cleared_rows.push(i);
		}
		cleared_rows.push(19);

		let mut previous = 0;
		for (index, cleared_row) in cleared_rows.iter().enumerate() {
			/*
			clear
			dont
			clear
			dont
			dont
			 */
			for i in previous..*cleared_row {
				self.board_with_current[i] = self.board_with_current[i+index];
			}
			previous = *cleared_row
		}
		
		
	}
}