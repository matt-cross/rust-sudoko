use colored::Colorize;
use fixedbitset::FixedBitSet;
use std::str::FromStr;

#[derive(Clone)]
enum Cell {
    Solved(usize),
    Unsolved(FixedBitSet),
}

impl Cell {
    fn new() -> Self {
        let mut bitset = FixedBitSet::with_capacity(9);
        bitset.set_range(.., true);
        Self::Unsolved(bitset)
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
    let board = Board::new();
    println!("Blank board:");
    for str in board.to_strs() {
        println!("{}", str);
    }
    println!("");

    let board2 = Board::from_str("5...27..9..41......1..5.3...92.6.8...5......66..7..29.8...7...2.......8...9..36..").unwrap();
    println!("Loaded board:");
    for str in board2.to_strs() {
        println!("{}", str);
    }
}
