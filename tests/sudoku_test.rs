use std::str::FromStr;
use sudoku_solver::sudoku::{is_valid, Sudoku};

#[test]
fn is_valid_test() {
    let sudoku = Sudoku::new([
        [0, 0, 4, 0, 5, 0, 0, 0, 0],
        [9, 5, 0, 7, 3, 4, 6, 0, 0],
        [0, 0, 3, 0, 2, 1, 0, 4, 9],
        [0, 3, 5, 0, 9, 0, 4, 8, 0],
        [0, 9, 0, 0, 0, 0, 0, 3, 0],
        [0, 7, 6, 0, 1, 0, 9, 2, 0],
        [3, 1, 0, 9, 7, 0, 2, 0, 0],
        [0, 0, 9, 1, 8, 2, 0, 0, 3],
        [0, 0, 0, 0, 6, 0, 1, 0, 0],
    ]);

    assert!(is_valid(&sudoku, 0, 0, 1));
    assert!(!is_valid(&sudoku, 0, 0, 4));
    assert!(!is_valid(&sudoku, 0, 0, 3));
    assert!(!is_valid(&sudoku, 0, 0, 5));
}

#[test]
fn solve_test() {
    let mut sudoku = Sudoku::new([
        [0, 0, 0, 4, 0, 6, 0, 3, 5],
        [0, 6, 0, 0, 0, 0, 0, 0, 0],
        [5, 0, 2, 3, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 4, 1, 0],
        [3, 0, 0, 0, 9, 0, 0, 7, 0],
        [4, 9, 0, 0, 0, 0, 8, 2, 0],
        [0, 0, 0, 2, 0, 9, 0, 0, 0],
        [0, 0, 8, 7, 1, 0, 0, 0, 0],
        [0, 7, 0, 0, 0, 0, 3, 0, 1],
    ]);

    let solution = Sudoku::new([
        [8, 1, 9, 4, 7, 6, 2, 3, 5],
        [7, 6, 3, 9, 2, 5, 1, 8, 4],
        [5, 4, 2, 3, 8, 1, 9, 6, 7],
        [6, 8, 7, 5, 3, 2, 4, 1, 9],
        [3, 2, 1, 8, 9, 4, 5, 7, 6],
        [4, 9, 5, 1, 6, 7, 8, 2, 3],
        [1, 3, 6, 2, 4, 9, 7, 5, 8],
        [9, 5, 8, 7, 1, 3, 6, 4, 2],
        [2, 7, 4, 6, 5, 8, 3, 9, 1],
    ]);

    sudoku.solve().unwrap();
    assert_eq!(sudoku, solution);
}

#[test]
fn from_string() {
    let input = "...4.6.35.6.......5.23...........41.3...9..7.49....82....2.9.....871.....7....3.1";
    let parsed = Sudoku::from_str(input).unwrap();

    let solution = Sudoku::new([
        [0, 0, 0, 4, 0, 6, 0, 3, 5],
        [0, 6, 0, 0, 0, 0, 0, 0, 0],
        [5, 0, 2, 3, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 4, 1, 0],
        [3, 0, 0, 0, 9, 0, 0, 7, 0],
        [4, 9, 0, 0, 0, 0, 8, 2, 0],
        [0, 0, 0, 2, 0, 9, 0, 0, 0],
        [0, 0, 8, 7, 1, 0, 0, 0, 0],
        [0, 7, 0, 0, 0, 0, 3, 0, 1],
    ]);

    assert_eq!(parsed, solution);
}