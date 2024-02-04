use crate::{engine::tetromino::TetrominoType, utils::terminal::colorize::{colorize_background, Colors}};


impl TetrominoType {
    pub fn get_text(&self) -> String {
        match self {
            Self::I => colorize_background(&"  ".to_string(), Colors::CYAN),
            Self::O => colorize_background(&"  ".to_string(), Colors::YELLOW),
            Self::T => colorize_background(&"  ".to_string(), Colors::PINK),
            Self::S => colorize_background(&"  ".to_string(), Colors::GREEN),
            Self::Z => colorize_background(&"  ".to_string(), Colors::RED),
            Self::J => colorize_background(&"  ".to_string(), Colors::BLUE),
            Self::L => colorize_background(&"  ".to_string(), Colors::GRAY),
            Self::None => "  ".to_string(),
        }
    }
}