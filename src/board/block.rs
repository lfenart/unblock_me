use super::Board;
use super::Direction;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Block {
    bitboard: u64,
    horizontal: bool,
}

impl Block {
    pub const WIDTH: usize = Board::WIDTH + 2;

    pub fn new(bitboard: u64) -> Self {
        Self {
            bitboard,
            horizontal: bitboard & (bitboard << 1) != 0,
        }
    }

    pub fn bitboard(self) -> u64 {
        self.bitboard
    }

    pub fn horizontal(self) -> bool {
        self.horizontal
    }

    pub fn shift(self, direction: Direction) -> Self {
        match direction {
            Direction::Left => self.shift_left(),
            Direction::Right => self.shift_right(),
            Direction::Up => self.shift_up(),
            Direction::Down => self.shift_down(),
        }
    }

    pub fn shift_right(mut self) -> Self {
        self.bitboard <<= 1;
        self
    }

    pub fn shift_left(mut self) -> Self {
        self.bitboard >>= 1;
        self
    }

    pub fn shift_up(mut self) -> Self {
        self.bitboard >>= Self::WIDTH;
        self
    }

    pub fn shift_down(mut self) -> Self {
        self.bitboard <<= Self::WIDTH;
        self
    }
}
