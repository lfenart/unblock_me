#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Action {
    block: u8,
    direction: Direction,
    amount: u8,
}

impl Action {
    pub fn new(block: u8, direction: Direction, amount: u8) -> Self {
        Self {
            block,
            direction,
            amount,
        }
    }

    pub fn block(self) -> u8 {
        self.block
    }

    pub fn direction(self) -> Direction {
        self.direction
    }

    pub fn amount(self) -> u8 {
        self.amount
    }
}

impl std::fmt::Display for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} ",
            if self.block < 9 {
                self.block + 1 + b'0'
            } else {
                self.block - 9 + b'a'
            } as char
        )?;
        let c = match self.direction {
            Direction::Left => 'l',
            Direction::Right => 'r',
            Direction::Up => 'u',
            Direction::Down => 'd',
        };
        for _ in 0..self.amount {
            write!(f, "{}", c)?;
        }
        Ok(())
    }
}
