mod breadth_first;

use super::board::{Action, Board};
pub use breadth_first::BreadthFirstSolver;

pub trait Solver {
    fn solve(&self, board: &Board) -> Option<Vec<Action>>;
}
