use crate::{engine::tetromino::TetrominoType, utils::terminal::colorize::Color};


impl TetrominoType {
    pub fn get_color(&self) -> Color {
        match self {
            Self::I => Color::CYAN,
            Self::O => Color::YELLOW,
            Self::T => Color::PINK,
            Self::S => Color::GREEN,
            Self::Z => Color::RED,
            Self::J => Color::BLUE,
            Self::L => Color::GRAY,
            Self::None => Color::NONE,
        }
    }
}