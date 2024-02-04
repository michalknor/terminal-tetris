use std::collections::HashSet;

#[derive(Clone, Copy, PartialEq)]
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

impl TetrominoType {
    fn get_blocks(&self) -> HashSet<(usize, usize)> {
        match self {
            TetrominoType::I => HashSet::from(
                [
                    (3, 21),
                    (4, 21),
                    (5, 21),
                    (6, 21),
                ]
            ),
            TetrominoType::O => HashSet::from(
                [
                    (4, 21),
                    (4, 20),
                    (5, 21),
                    (5, 20),
                ]
            ),
            TetrominoType::T => HashSet::from(
                [
                    (4, 21),
                    (5, 21),
                    (5, 20),
                    (6, 21),
                ]
            ),
            TetrominoType::S => HashSet::from(
                [
                    (4, 20),
                    (5, 20),
                    (5, 21),
                    (6, 21),
                ]
            ),
            TetrominoType::Z => HashSet::from(
                [
                    (4, 21),
                    (5, 21),
                    (5, 20),
                    (6, 20),
                ]
            ),
            TetrominoType::J => HashSet::from(
                [
                    (5, 22),
                    (5, 21),
                    (5, 20),
                    (4, 20),
                ]
            ),
            TetrominoType::L => HashSet::from(
                [
                    (4, 22),
                    (4, 21),
                    (4, 20),
                    (5, 20),
                ]
            ),
            TetrominoType::None => HashSet::new()
        }
    }
}

pub struct Tetromino {
    tetromino_type: TetrominoType,
    blocks: HashSet<(usize, usize)>,
}


impl Tetromino {
    pub fn new_bag() -> Vec<Self> {
        vec![
            Tetromino {tetromino_type: TetrominoType::I, blocks: TetrominoType::I.get_blocks()},
            Tetromino {tetromino_type: TetrominoType::O, blocks: TetrominoType::O.get_blocks()},
            Tetromino {tetromino_type: TetrominoType::T, blocks: TetrominoType::T.get_blocks()},
            Tetromino {tetromino_type: TetrominoType::S, blocks: TetrominoType::S.get_blocks()},
            Tetromino {tetromino_type: TetrominoType::Z, blocks: TetrominoType::Z.get_blocks()},
            Tetromino {tetromino_type: TetrominoType::J, blocks: TetrominoType::J.get_blocks()},
            Tetromino {tetromino_type: TetrominoType::L, blocks: TetrominoType::L.get_blocks()},
        ]
    }

    pub fn get_blocks(&self) -> &HashSet<(usize, usize)> {
        &self.blocks
    }

    pub fn get_tetromino_type(&self) -> TetrominoType {
        self.tetromino_type
    }

    pub fn fall(&mut self) {
        self.blocks = self
            .blocks
            .iter()
            .map(|&item| (item.0, item.1 - 1))
            .collect();
    }
}