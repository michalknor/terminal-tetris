use rand::prelude::*;


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
    fn get_blocks(&self) -> [(usize, usize); 4] {
        match self {
            TetrominoType::I => [
                (3, 21),
                (4, 21),
                (5, 21),
                (6, 21),
            ],
            TetrominoType::O => [
                (4, 21),
                (4, 20),
                (5, 21),
                (5, 20),
            ],
            TetrominoType::T => [
                (4, 21),
                (5, 21),
                (5, 20),
                (6, 21),
            ],
            TetrominoType::S => [
                (4, 20),
                (5, 20),
                (5, 21),
                (6, 21),
            ],
            TetrominoType::Z => [
                (4, 21),
                (5, 21),
                (5, 20),
                (6, 20),
            ],
            TetrominoType::J => [
                (5, 22),
                (5, 21),
                (5, 20),
                (4, 20),
            ],
            TetrominoType::L => [
                (4, 22),
                (4, 21),
                (4, 20),
                (5, 20),
            ],
            TetrominoType::None => panic!("asd")
        }
    }
}


pub struct Tetromino {
    tetromino_type: TetrominoType,
    blocks: [(usize, usize); 4],
}


impl Tetromino {
    pub fn new_bag(rng: &mut StdRng) -> Vec<Self> {
        let mut bag: Vec<Tetromino> = vec![
            Tetromino {tetromino_type: TetrominoType::I, blocks: TetrominoType::I.get_blocks()},
            Tetromino {tetromino_type: TetrominoType::O, blocks: TetrominoType::O.get_blocks()},
            Tetromino {tetromino_type: TetrominoType::T, blocks: TetrominoType::T.get_blocks()},
            Tetromino {tetromino_type: TetrominoType::S, blocks: TetrominoType::S.get_blocks()},
            Tetromino {tetromino_type: TetrominoType::Z, blocks: TetrominoType::Z.get_blocks()},
            Tetromino {tetromino_type: TetrominoType::J, blocks: TetrominoType::J.get_blocks()},
            Tetromino {tetromino_type: TetrominoType::L, blocks: TetrominoType::L.get_blocks()},
        ];

        let mut bag_shuffled: Vec<Tetromino> = Vec::new();

        while bag.len() != 0 {
            let tetromino: Tetromino = bag.remove(rng.gen_range(0..bag.len()));
            bag_shuffled.push(tetromino);
        }

        bag_shuffled
    }

    pub fn get_blocks(&self) -> [(usize, usize); 4] {
        self.blocks
    }

    pub fn get_tetromino_type(&self) -> TetrominoType {
        self.tetromino_type
    }

    pub fn fall(&mut self) {
        for (i, &(x, y)) in self.blocks.clone().iter().enumerate() {
            self.blocks[i] = (x, y-1);
        }
    }
}