use crate::Board;
use crate::Cell;
use crate::Strategy;

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

        // Ugh, the nesting in this is really deep.  It would be nice
        // to reorganize this in a more linear manner rather than
        // nested loops and if's.

        for group in Board::all_groups() {
            // We only need to check up to index 7 in the group as we
            // will then proceed to look for a match.  It doesn't
            // matter to us if the last cell in the group is an
            // unmatched pair.
            for idx1 in 0..8 {
                let cell1 = board.cells[group[idx1]].clone();

                if let Cell::Unsolved(ref bitset) = cell1 {
                    if bitset.count_ones(..) == 2 {
                        // idx1 is a pair, look for a matching pair in the rest of the group.
                        for idx2 in idx1+1..9 {
                            let cell2 = board.cells[group[idx2]].clone();

                            if cell2 == cell1 {
                                // We found a matching naked pair.
                                // Remove the digits in this pair from
                                // all other cells in the group.

                                for digit_idx in bitset.ones() {
                                    let digit = digit_idx + 1;

                                    for idx in 0..9 {
                                        if idx != idx1 && idx != idx2 {
                                            board.cells[group[idx]].remove(digit).unwrap();
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        board
    }
}
