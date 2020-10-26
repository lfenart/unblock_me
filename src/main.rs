mod board;
mod solver;

use std::io::stdin;

use board::Board;
use solver::{BreadthFirstSolver, Solver};

fn main() {
    let board = Board::read_from(&mut stdin().lock());
    let actions = BreadthFirstSolver.solve(&board);
    match actions {
        None => eprintln!("No solution"),
        Some(actions) => {
            for action in actions {
                println!("{}", action);
            }
        }
    }
}
