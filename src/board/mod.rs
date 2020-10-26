mod action;
mod block;

pub use action::Action;
pub use action::Direction;
use block::Block;

use std::io::BufRead;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Board {
    occupied: u64,
    blocks: Vec<Block>,
}

impl Board {
    pub const HEIGHT: usize = 6;
    pub const WIDTH: usize = 6;

    pub fn new(array: &[u8; Self::WIDTH * Self::HEIGHT], n: u8) -> Self {
        let mut board = Board {
            occupied: 0xff818181818181ff,
            blocks: Vec::with_capacity(n as usize),
        };
        let mut bitboards = vec![0; n as usize];
        for i in 0..Self::HEIGHT {
            for j in 0..Self::WIDTH {
                let value = array[Self::WIDTH * i + j];
                if value != 0 {
                    bitboards[(value - 1) as usize] |= 1 << (Block::WIDTH * (i + 1) + j + 1);
                }
            }
        }
        for bitboard in bitboards {
            board.blocks.push(Block::new(bitboard));
            board.occupied |= bitboard;
        }
        board
    }

    pub fn read_from(reader: &mut impl BufRead) -> Self {
        let mut result = [0; Board::WIDTH * Board::HEIGHT];
        let mut n = 0;
        for i in 0..Board::HEIGHT {
            let mut buf = String::with_capacity(Board::WIDTH);
            reader.read_line(&mut buf).unwrap();
            for j in 0..Board::WIDTH {
                let x = match buf.chars().nth(j).unwrap() as u8 {
                    x @ b'0'..=b'9' => x - b'0',
                    x => x - b'a' + 10,
                };
                result[i * Board::WIDTH + j] = x;
                if x > n {
                    n = x;
                }
            }
        }
        Self::new(&result, n)
    }

    pub fn get_actions(&self) -> Vec<Action> {
        let mut actions = Vec::new();
        for (index, block) in self.blocks.iter().enumerate() {
            for &direction in if block.horizontal() {
                [Direction::Right, Direction::Left]
            } else {
                [Direction::Up, Direction::Down]
            }
            .iter()
            {
                let occupied = self.occupied ^ block.bitboard();
                let mut block = block.shift(direction);
                let mut amount = 0;
                while occupied & block.bitboard() == 0 {
                    amount += 1;
                    actions.push(Action::new(index as u8, direction, amount));
                    block = block.shift(direction);
                }
            }
        }
        actions
    }

    pub fn play(&mut self, action: Action) {
        let block = &mut self.blocks[action.block() as usize];
        self.occupied ^= block.bitboard();
        let mut new_block = *block;
        for _ in 0..action.amount() {
            new_block = new_block.shift(action.direction());
        }
        *block = new_block;
        self.occupied |= block.bitboard();
    }

    pub fn is_solved(&self) -> bool {
        (self.occupied ^ self.blocks[0].bitboard()) & 0x7e000000
            < self.blocks[0].bitboard() & 0x7e000000
    }
}
