use crate::Board;
use crate::Cell;
use crate::Strategy;

pub struct RemoveSolvedFromNeighbors;

impl Strategy for RemoveSolvedFromNeighbors {
    fn new() -> Box<dyn Strategy> {
        Box::new(RemoveSolvedFromNeighbors {})
    }

    fn name(&self) -> String {
        String::from("RemoveSolvedFromNeighbors")
    }

    fn apply(&self, board: &Board) -> Board {
        let mut result = board.clone();

        for idx in 0..81 {
            if let Cell::Solved(digit) = board.cells[idx] {
                let neighbors = Board::all_neighbors(idx);

                for neighbor in neighbors {
                    result.cells[neighbor].remove(digit).unwrap();
                }
            }
        }

        result
    }
}

