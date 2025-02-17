use super::*;

#[test]
fn test_empty_cell_create() {
    assert_eq!(Cell::new(),
               Cell::from_digits([1,2,3,4,5,6,7,8,9]));
}

#[test]
fn test_cell_remove() {
    let mut c = Cell::new();
    c.remove(4).unwrap();
    assert_eq!(c, Cell::from_digits([1,2,3,5,6,7,8,9]));
}

#[test]
fn test_cell_remove_to_solved() {
    let mut c = Cell::new();
    c.remove(1).unwrap();
    c.remove(2).unwrap();
    c.remove(3).unwrap();
    c.remove(4).unwrap();
    c.remove(5).unwrap();
    c.remove(6).unwrap();
    // Leave 7 in
    c.remove(8).unwrap();
    c.remove(9).unwrap();
    assert_eq!(c, Cell::Solved(7));
}

#[test]
fn test_cell_remove_from_solved() -> Result<(), String> {
    let mut c = Cell::Solved(4);

    c.remove(1)?;
    c.remove(2)?;
    c.remove(3)?;
    c.remove(5)?;
    assert!(c.remove(4).is_err());
    c.remove(6)?;
    c.remove(7)?;
    c.remove(8)?;
    c.remove(9)?;

    Ok(())
}

#[test]
fn test_empty_board() {
    let b = Board::new();
    let empty_cell = Cell::new();

    for cell in b.cells {
        assert_eq!(cell, empty_cell);
    }
}

#[test]
fn test_loaded_board() {
    let b = Board::from_str("123......456......789.........123......456......789.........123......456......789").unwrap();
    let empty_cell = Cell::new();

    // Check all of the top 3 boxes
    assert_eq!(b.cells[0], Cell::from('1'));
    assert_eq!(b.cells[1], Cell::from('2'));
    assert_eq!(b.cells[2], Cell::from('3'));
    assert_eq!(b.cells[3], empty_cell);
    assert_eq!(b.cells[4], empty_cell);
    assert_eq!(b.cells[5], empty_cell);
    assert_eq!(b.cells[6], empty_cell);
    assert_eq!(b.cells[7], empty_cell);
    assert_eq!(b.cells[8], empty_cell);

    assert_eq!(b.cells[9], Cell::from('4'));
    assert_eq!(b.cells[10], Cell::from('5'));
    assert_eq!(b.cells[11], Cell::from('6'));
    assert_eq!(b.cells[12], empty_cell);
    assert_eq!(b.cells[13], empty_cell);
    assert_eq!(b.cells[14], empty_cell);
    assert_eq!(b.cells[15], empty_cell);
    assert_eq!(b.cells[16], empty_cell);
    assert_eq!(b.cells[17], empty_cell);

    assert_eq!(b.cells[18], Cell::from('7'));
    assert_eq!(b.cells[19], Cell::from('8'));
    assert_eq!(b.cells[20], Cell::from('9'));
    assert_eq!(b.cells[21], empty_cell);
    assert_eq!(b.cells[22], empty_cell);
    assert_eq!(b.cells[23], empty_cell);
    assert_eq!(b.cells[24], empty_cell);
    assert_eq!(b.cells[25], empty_cell);
    assert_eq!(b.cells[26], empty_cell);

    // Spot check a few others
    assert_eq!(b.cells[37], empty_cell);
    assert_eq!(b.cells[40], Cell::from('5'));
    assert_eq!(b.cells[43], empty_cell);

    assert_eq!(b.cells[74], empty_cell);
    assert_eq!(b.cells[77], empty_cell);
    assert_eq!(b.cells[80], Cell::from('9'));
}

#[test]
fn test_row_neighbors() {
    let rn = Board::row_neighbors(14);
    assert_eq!(rn, [9,10,11,12,13,15,16,17]);

    let rn = Board::row_neighbors(79);
    assert_eq!(rn, [72,73,74,75,76,77,78,80]);
}

#[test]
fn test_col_neighbors() {
    let rn = Board::column_neighbors(14);
    assert_eq!(rn, [5,23,32,41,50,59,68,77]);

    let rn = Board::column_neighbors(79);
    assert_eq!(rn, [7,16,25,34,43,52,61,70]);
}

#[test]
fn test_box_neighbors() {
    let rn = Board::box_neighbors(14);
    assert_eq!(rn, [3,4,5,12,13,21,22,23]);

    let rn = Board::box_neighbors(79);
    assert_eq!(rn, [60,61,62,69,70,71,78,80]);
}

#[test]
fn test_empty_board_valid() {
    assert!(Board::new().valid());
}

#[test]
fn test_loaded_board_valid() {
    let b = Board::from_str("5...27..9..41......1..5.3...92.6.8...5......66..7..29.8...7...2.......8...9..36..").unwrap();
    assert!(b.valid());
}

#[test]
fn test_loaded_board_invalid() {
    // 5 at 0 (0,0) and at 72 (8,0)
    let b = Board::from_str("5...27..9..41......1..5.3...92.6.8...5......66..7..29.8...7...2.......8.5.9..36..").unwrap();
    assert!(!b.valid());
}

#[test]
fn test_empty_board_unsolved() {
    assert!(!Board::new().solved());
}

#[test]
fn test_solved() {
    // The most trivial solved sudoku board
    let b = Board::from_str("123456789456789123789123456234567891567891234891234567345678912678912345912345678").unwrap();
    assert!(b.solved());
}
