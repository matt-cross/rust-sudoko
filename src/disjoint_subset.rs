use crate::Board;
use crate::Strategy;
use std::collections::HashMap;

pub struct NakedPair;

impl Strategy for NakedPair {
    fn new() -> Box<dyn Strategy> {
        Box::new(NakedPair {})
    }

    fn name(&self) -> String {
        String::from("NakedPair")
    }

    fn apply(&self, board_in: &Board) -> Board {
        let mut board = board_in.clone();

        for group in Board::all_groups() {
            // Build up a set of all cells with two possible digits,
            // and a count of the number of cells like that in this
            // group that match that.
            let mut naked_pair_counts = HashMap::new();
            for cell_and_loc in board.get_cells(&group) {
                naked_pair_counts.entry(cell_and_loc.cell.clone()).and_modify(|counter| *counter += 1).or_insert(1);
            }

            for (cell, count) in naked_pair_counts {
                // If there are two of this pair, we know they must be
                // the only cells that have these digits in this
                // group.
                if count == 2 {
                    // The two digits in this pair can be removed from
                    // all cells in this row that are not part of this
                    // pair.
                    let digits = cell.digits();
                    assert!(digits.len() == 2);
                    for board_idx in &group {
                        let bcell = &mut board.cells[*board_idx];
                        if *bcell != cell {
                            digits.iter().for_each(|digit| board.cells[*board_idx].remove(*digit).unwrap());
                        }
                    }
                }
            }
        }

        board
    }
}

