use std::collections::HashSet;

#[derive(Clone, Copy)]
pub enum TetrominoType {
    I,
    O,
    T,
    S,
    Z,
    J,
    L,
    None,
}

pub struct Tetromino {
    tetromino: TetrominoType,
    position: HashSet<(usize, usize)>,
}


impl Tetromino {
    pub fn new_bag() -> [Self; 7] {
		[
            Tetromino {tetromino: TetrominoType::I, position: HashSet::new()},
            Tetromino {tetromino: TetrominoType::O, position: HashSet::new()},
            Tetromino {tetromino: TetrominoType::T, position: HashSet::new()},
            Tetromino {tetromino: TetrominoType::S, position: HashSet::new()},
            Tetromino {tetromino: TetrominoType::Z, position: HashSet::new()},
            Tetromino {tetromino: TetrominoType::J, position: HashSet::new()},
            Tetromino {tetromino: TetrominoType::L, position: HashSet::new()},
        ]
    }
}