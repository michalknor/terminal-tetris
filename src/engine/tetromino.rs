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
    fn get_rotation_states(&self) -> [[(i8, i8); 4]; 4] {
        match self {
            TetrominoType::I => [
                [
                    (3, 21),
                    (4, 21),
                    (5, 21),
                    (6, 21),
                ],
                [
                    (5, 22),
                    (5, 21),
                    (5, 20),
                    (5, 19),
                ],
                [
                    (3, 20),
                    (4, 20),
                    (5, 20),
                    (6, 20),
                ],
                [
                    (4, 22),
                    (4, 21),
                    (4, 20),
                    (4, 19),
                ]
            ],
            TetrominoType::O => [
                [
                    (4, 21),
                    (4, 20),
                    (5, 21),
                    (5, 20),
                ],
                [
                    (4, 21),
                    (4, 20),
                    (5, 21),
                    (5, 20),
                ],
                [
                    (4, 21),
                    (4, 20),
                    (5, 21),
                    (5, 20),
                ],
                [
                    (4, 21),
                    (4, 20),
                    (5, 21),
                    (5, 20),
                ]
            ],
            TetrominoType::T => [
                [
                    (4, 21),
                    (5, 21),
                    (6, 21),
                    (5, 20),
                ],
                [
                    (5, 22),
                    (4, 21),
                    (5, 21),
                    (5, 20),
                ],
                [
                    (5, 22),
                    (4, 21),
                    (5, 21),
                    (6, 21),
                ],
                [
                    (5, 22),
                    (5, 21),
                    (6, 21),
                    (5, 20),
                ]
            ],
            TetrominoType::S => [
                [
                    (5, 21),
                    (6, 21),
                    (4, 20),
                    (5, 20),
                ],
                [
                    (5, 21),
                    (5, 20),
                    (6, 20),
                    (6, 19),
                ],
                [
                    (5, 20),
                    (6, 20),
                    (4, 19),
                    (5, 19),
                ],
                [
                    (4, 21),
                    (4, 20),
                    (5, 20),
                    (5, 19),
                ],
            ],
            TetrominoType::Z => [
                [
                    (4, 21),
                    (5, 21),
                    (5, 20),
                    (6, 20),
                ],
                [
                    (6, 21),
                    (5, 20),
                    (6, 20),
                    (5, 19),
                ],
                [
                    (4, 20),
                    (5, 20),
                    (5, 19),
                    (6, 19),
                ],
                [
                    (5, 21),
                    (4, 20),
                    (5, 20),
                    (4, 19),
                ]
            ],
            TetrominoType::J => [
                [
                    (5, 22),
                    (5, 21),
                    (4, 20),
                    (5, 20),
                ],
                [
                    (4, 22),
                    (4, 21),
                    (5, 21),
                    (6, 21),
                ],
                [
                    (5, 22),
                    (6, 22),
                    (5, 21),
                    (5, 20),
                ],
                [
                    (4, 21),
                    (5, 21),
                    (6, 21),
                    (6, 20),
                ]
            ],
            TetrominoType::L => [
                [
                    (5, 22),
                    (5, 21),
                    (5, 20),
                    (6, 20),
                ],
                [
                    (4, 21),
                    (5, 21),
                    (6, 21),
                    (4, 20),
                ],
                [
                    (4, 22),
                    (5, 22),
                    (5, 21),
                    (5, 20),
                ],
                [
                    (6, 22),
                    (4, 21),
                    (5, 21),
                    (6, 21),
                ]
            ],
            TetrominoType::None => panic!("None tetromino does not have blocks!")
        }
    }
}


pub struct Tetromino {
    tetromino_type: TetrominoType,
    rotation_states: [[(i8, i8); 4]; 4],
    rotation_index: u8,
}


impl Tetromino {
    pub fn new_bag(rng: &mut StdRng) -> Vec<Self> {
        let mut bag: Vec<Tetromino> = vec![
            Tetromino {tetromino_type: TetrominoType::I, rotation_states: TetrominoType::I.get_rotation_states(), rotation_index: 0},
            Tetromino {tetromino_type: TetrominoType::O, rotation_states: TetrominoType::O.get_rotation_states(), rotation_index: 0},
            Tetromino {tetromino_type: TetrominoType::T, rotation_states: TetrominoType::T.get_rotation_states(), rotation_index: 0},
            Tetromino {tetromino_type: TetrominoType::S, rotation_states: TetrominoType::S.get_rotation_states(), rotation_index: 0},
            Tetromino {tetromino_type: TetrominoType::Z, rotation_states: TetrominoType::Z.get_rotation_states(), rotation_index: 0},
            Tetromino {tetromino_type: TetrominoType::J, rotation_states: TetrominoType::J.get_rotation_states(), rotation_index: 0},
            Tetromino {tetromino_type: TetrominoType::L, rotation_states: TetrominoType::L.get_rotation_states(), rotation_index: 0},
        ];

        let mut bag_shuffled: Vec<Tetromino> = Vec::new();

        while !bag.is_empty() {
            let tetromino: Tetromino = bag.remove(rng.gen_range(0..bag.len()));
            bag_shuffled.push(tetromino);
        }

        bag_shuffled
    }

    pub fn get_blocks(&self) -> [(i8, i8); 4] {
        self.rotation_states[self.rotation_index as usize]
    }

    pub fn get_tetromino_type(&self) -> TetrominoType {
        self.tetromino_type
    }

	pub fn move_left(&mut self) {
        for i in 0..self.rotation_states.len() {
            for (j, &(x, y)) in self.rotation_states[i].clone().iter().enumerate() {
                self.rotation_states[i][j] = (x-1, y);
            }
        }
	}

	pub fn move_right(&mut self) {
        for i in 0..self.rotation_states.len() {
            for (j, &(x, y)) in self.rotation_states[i].clone().iter().enumerate() {
                self.rotation_states[i][j] = (x+1, y);
            }
        }
	}

    pub fn fall(&mut self) {
        for i in 0..self.rotation_states.len() {
            for (j, &(x, y)) in self.rotation_states[i].clone().iter().enumerate() {
                self.rotation_states[i][j] = (x, y-1);
            }
        }
    }

	pub fn rotate_left(&mut self) {
		self.rotation_index = ((self.rotation_index as usize + 1) % self.rotation_states.len()) as u8
	}

	pub fn rotate_right(&mut self) {
		self.rotation_index = ((self.rotation_index as usize + self.rotation_states.len() - 1) % self.rotation_states.len()) as u8
	}
}