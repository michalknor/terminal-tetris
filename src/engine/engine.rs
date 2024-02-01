pub struct Game {
	board: [[i32; 10]; 20]
}

impl Game {
	pub fn new() -> Game {
		Game {
			board: [[0; 10]; 20]
		}
	}

	pub fn get_board(&self) -> &[[i32; 10]; 20] {
		&self.board
	}
}