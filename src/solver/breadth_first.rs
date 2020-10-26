use std::collections::{HashSet, VecDeque};

use super::Solver;
use crate::board::{Action, Board};

pub struct BreadthFirstSolver;

impl Solver for BreadthFirstSolver {
    fn solve(&self, board: &Board) -> Option<Vec<Action>> {
        let mut set = HashSet::new();
        let mut queue = VecDeque::new();
        queue.push_back((board.clone(), vec![]));
        while !queue.is_empty() {
            let (board, actions) = queue.pop_front().unwrap();
            if board.is_solved() {
                return Some(actions);
            }
            if set.contains(&board) {
                continue;
            }
            let action_list = board.get_actions();
            let first_move = actions.is_empty();
            for action in action_list {
                if !first_move && *actions.last().unwrap() == action {
                    continue;
                }
                let mut new_board = board.clone();
                new_board.play(action);
                let mut new_actions = actions.clone();
                new_actions.push(action);
                queue.push_back((new_board, new_actions));
            }
            set.insert(board);
        }
        None
    }
}
