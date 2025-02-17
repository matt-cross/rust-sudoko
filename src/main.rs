use colored::Colorize;
use fixedbitset::FixedBitSet;
use std::str::FromStr;

#[cfg(test)]
mod tests;

#[derive(Clone,PartialEq,Debug)]
enum Cell {
    Solved(usize),
    Unsolved(FixedBitSet),
}

impl Cell {
    // Make a new empty cell that can hold any digit
    fn new() -> Self {
        let mut bitset = FixedBitSet::with_capacity(9);
        bitset.set_range(.., true);
        Self::Unsolved(bitset)
    }

    // Make a new empty cell that can hold the digits in the passed-in
    // iterator.
    fn from_digits<I>(digits: I) -> Self
    where
        I: IntoIterator<Item = usize>
    {
        let mut bitset = FixedBitSet::with_capacity(9);
        for digit in digits {
            assert!(digit >= 1 && digit <= 9);
            bitset.set(digit-1, true);
        }

        if bitset.count_ones(..) == 1 {
            // Only one possible value - this is a solved cell
            let digit = bitset.ones().next().unwrap() + 1;
            Self::Solved(digit)
        } else {
            Self::Unsolved(bitset)
        }
    }

    // Remove the digit from the set of possible digits this cell can
    // hold.  If the cell is now down to just one possible digit,
    // transition it to solved.
    fn remove(&mut self, digit: usize) -> Result<(), String> {
        if digit < 1 || digit > 9 {
            return Err(format!("Cell::remove called with invalid digit {}", digit));
        }

        match self {
            Self::Unsolved(ref mut bitset) => {
                bitset.set(digit-1, false);
                if bitset.count_ones(..) == 1 {
                    let digit = bitset.ones().next().unwrap() + 1;
                    *self = Self::Solved(digit);
                }
                Ok(())
            },

            // If some strategy is trying to remove a solved digit, that is an error.
            Self::Solved(d) if *d == digit => Err(format!("Cell::remove asked to remove solved digit {}", digit)),

            // On the other hand, trying to remove any digit other than the currently solved one is OK.
            Self::Solved(_) => Ok(()),
        }
    }

    fn to_strs(&self) -> [String; 3] {
        match self {
            Self::Solved(value) => [String::from("   "),
                                    format!(" {value} "),
                                    String::from("   ")],
            Self::Unsolved(bits) => {
                let mut result = Vec::new();

                for idx in 0..9 {
                    if bits[idx] {
                        result.push(format!("{}", idx+1).dimmed());
                    } else {
                        result.push(format!(" ").into());
                    }
                }

                [format!("{}{}{}", result[0], result[1], result[2]),
                 format!("{}{}{}", result[3], result[4], result[5]),
                 format!("{}{}{}", result[6], result[7], result[8])]
            }
        }
    }
}

impl From<char> for Cell {
    fn from(ch: char) -> Self {
        match ch {
            '0'..='9' => Self::Solved(ch.to_digit(10).unwrap().try_into().unwrap()),
            _ => Self::new(),
        }
    }
}

struct Board {
    cells: [Cell; 81],
}

impl Board {
    fn new() -> Self {
        Board {
            cells: core::array::from_fn(|_| Cell::new()),
        }
    }

    fn to_strs(&self) -> Vec<String> {
        // A cell in the sudoku grid is displayed as a 3x3 cell so we
        // can display either the correct solved value or what is
        // currently known about the possible values this cell can
        // hold (sometimes called pencil marks).
        let cell_strs = self.cells.clone().map(|c| c.to_strs());

        // A complete grid consists of 81 cells, arranged in boxes of
        // 3x3 cells.  Each cell will be a 3x3 grid of numbers as
        // described above.  There will be single lines dividing the
        // individual cells, and double lines dividing the boxes.
        //
        // This shows a representation of the desired output, with
        // cell indices in some of the cells to give an idea for how
        // it will be laid out in the array:
        //
        // ###|###|###||###|###|###||###|###|###
        // #0#|#1#|#2#||#3#|#4#|#5#||#6#|#7#|#8#
        // ###|###|###||###|###|###||###|###|###
        // ---+---+---++---+---+---++---+---+---
        // ###|###|###||###|###|###||###|###|###
        // #9#|10#|11#||12#|13#|14#||15#|16#|17#
        // ###|###|###||###|###|###||###|###|###
        // ---+---+---++---+---+---++---+---+---
        // ###|###|###||###|###|###||###|###|###
        // 18#|###|###||###|###|###||###|###|###
        // ###|###|###||###|###|###||###|###|###
        // ===+===+===++===+===+===++===+===+===
        // ###|###|###||###|###|###||###|###|###
        // 27#|###|###||###|###|###||###|###|###
        // ###|###|###||###|###|###||###|###|###
        // ---+---+---++---+---+---++---+---+---
        // ###|###|###||###|###|###||###|###|###
        // 36#|###|###||###|###|###||###|###|###
        // ###|###|###||###|###|###||###|###|###
        // ---+---+---++---+---+---++---+---+---
        // ###|###|###||###|###|###||###|###|###
        // 45#|###|###||###|###|###||###|###|###
        // ###|###|###||###|###|###||###|###|###
        // ===+===+===++===+===+===++===+===+===
        // ###|###|###||###|###|###||###|###|###
        // 54#|###|###||###|###|###||###|###|###
        // ###|###|###||###|###|###||###|###|###
        // ---+---+---++---+---+---++---+---+---
        // ###|###|###||###|###|###||###|###|###
        // 63#|###|###||###|###|###||###|###|###
        // ###|###|###||###|###|###||###|###|###
        // ---+---+---++---+---+---++---+---+---
        // ###|###|###||###|###|###||###|###|###
        // 72#|73#|74#||75#|76#|77#||78#|79#|80#
        // ###|###|###||###|###|###||###|###|###
        //
        // There are 35 lines generated, including separators.  When
        // lineno%4 == 3, there will be a separator; when lineno%12 ==
        // 11 it wiull be a double separator.

        let mut results = Vec::<String>::new();

        for lineno in 0..35 {
            if (lineno % 12) == 11 {
                results.push(String::from("===+===+===++===+===+===++===+===+==="));
            } else if (lineno % 4) == 3 {
                results.push(String::from("---+---+---++---+---+---++---+---+---"));
            } else {
                let s = (lineno/4) * 9; // first cell in this line of output
                let r = lineno%4; // row in each cell in this line of output
                results.push(format!("{}|{}|{}||{}|{}|{}||{}|{}|{}",
                                     cell_strs[s+0][r], cell_strs[s+1][r], cell_strs[s+2][r],
                                     cell_strs[s+3][r], cell_strs[s+4][r], cell_strs[s+5][r],
                                     cell_strs[s+6][r], cell_strs[s+7][r], cell_strs[s+8][r]));
            }
        }

        results
    }

    // Returns true if this board is valid, false otherwise.  Valid
    // means that it does not violate the basic sudoko constraints of
    // solved cells having a duplicate (solved) digit in the rest of
    // that cell's row, column, and box neighbors.
    fn valid(&self) -> bool {
        // This checks each pair of cells twice and could be
        // optimized.
        for idx in 0..81 {
            let cell = &self.cells[idx];
            if let Cell::Solved(_) = cell {
                for nidx in Self::all_neighbors(idx) {
                    let n = &self.cells[nidx];
                    if cell == n {
                        return false;
                    }
                }
            }
        }

        // If we get here no constraints were violated, so this board
        // is valid.
        true
    }

    fn solved(&self) -> bool {
        self.valid()
            && self.cells
                   .iter()
                   .all(|cell|
                        match cell {
                            Cell::Solved(_) => true,
                            _ => false,
                        })
    }

    // Given a cell index, return a vector of cell indices that are
    // the other cells in this cell's row.
    fn row_neighbors(idx: usize) -> Vec<usize> {
        assert!(idx < 81);

        let row_start = idx - (idx % 9);
        let mut result = Vec::new();
        for n_idx in row_start..row_start+9 {
            if n_idx != idx {
                result.push(n_idx);
            }
        }

        result
    }

    // Given a cell index, return a vector of cell indices that are
    // the other cells in this cell's column.
    fn column_neighbors(idx: usize) -> Vec<usize> {
        assert!(idx < 81);

        let col_start = idx % 9;
        let mut result = Vec::new();
        for row in 0..9 {
            let n_idx = col_start + row*9;
            if n_idx != idx {
                result.push(n_idx);
            }
        }

        result
    }

    // Given a cell index, return a vector of cell indices that are
    // the other cells in this cell's box.
    fn box_neighbors(idx: usize) -> Vec<usize> {
        assert!(idx < 81);

        // Row and column of this cell
        let row = idx/9;
        let col = idx%9;

        // The start row and column of the box holding this cell
        let box_row = row - row%3;
        let box_col = col - col%3;

        let mut result = Vec::new();
        for row in 0..3 {
            for col in 0..3 {
                let n_idx = (row + box_row) * 9 + box_col + col;
                if n_idx != idx {
                    result.push(n_idx);
                }
            }
        }

        result
    }

    fn all_neighbors(idx: usize) -> Vec<usize> {
        let mut result = Self::row_neighbors(idx);
        result.extend(Self::column_neighbors(idx).iter());
        result.extend(Self::box_neighbors(idx).iter());
        result.sort_unstable();
        result.dedup();
        result
    }
}

#[derive(Debug, PartialEq, Eq)]
struct ParseBoardError;

impl FromStr for Board {
    type Err = ParseBoardError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() == 81 {
            Ok(Self {
                cells: core::array::from_fn(|idx| Cell::from(s.chars().nth(idx).unwrap())),
            })
        } else {
            Err(ParseBoardError)
        }
    }
}

fn main() {
    let board = Board::from_str("5...27..9..41......1..5.3...92.6.8...5......66..7..29.8...7...2.......8...9..36..").unwrap();
    println!("Loaded board:");
    for str in board.to_strs() {
        println!("{}", str);
    }

    for cell in [0, 4, 5, 10, 13, 25, 37, 39, 41, 64, 72, 80] {
        println!("");
        println!("row neighbors of {}: {:?}", cell, Board::row_neighbors(cell));
        println!("column neighbors of {}: {:?}", cell, Board::column_neighbors(cell));
        println!("box neighbors of {}: {:?}", cell, Board::box_neighbors(cell));
    }
}
