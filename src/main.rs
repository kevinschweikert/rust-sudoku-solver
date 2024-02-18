use std::env;
use std::str::FromStr;
use sudoku_solver::sudoku::Sudoku;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = &args[1];

    let mut sudoku = Sudoku::from_str(input).expect("No valid input found");
    sudoku.solve().expect("Grid must be solvable");
    println!("{}", sudoku);
}
