use crossterm::event::KeyCode;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};

pub enum KeyPressed {
	ROTATE,
	LEFT,
	RIGHT,
	DOWN,
	NONE,
}

pub struct Game {
	board: [[i32; 10]; 20],
	rx_key_event: UnboundedReceiver<KeyCode>,
}

impl Game {
	pub fn new(
		rx_key_event: UnboundedReceiver<KeyCode>
	) -> Self {
		let board = [[0; 10]; 20];
		Self {
			board,
			rx_key_event,
		}
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

	pub fn get_board(&self) -> &[[i32; 10]; 20] {
		&self.board
	}
}