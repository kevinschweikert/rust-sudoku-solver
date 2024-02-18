pub mod sudoku {
    use std::fmt;
    use std::str::FromStr;

    #[derive(Debug, PartialEq)]
    pub struct Sudoku([[usize; 9]; 9]);

    #[derive(Debug, PartialEq, Eq)]
    pub struct ParseSudokuError(String);

    #[derive(Debug)]
    pub struct SolveError;

    impl Sudoku {
        pub fn new(data: [[usize; 9]; 9]) -> Self {
            Sudoku(data)
        }

        /// Solves the given sudoku
        pub fn solve(&mut self) -> Result<(), SolveError> {
            solve_recursive(self, 0, 0)
        }
    }

    /// Checks if a given number `k` is valid in the row `r` and column `c`
    pub fn is_valid(sudoku: &Sudoku, r: usize, c: usize, k: usize) -> bool {
        let grid = sudoku.0;
        let not_in_row = !grid[r].contains(&k);
        let not_in_column = (0..9).all(|i| grid[i][c] != k);
        let not_in_box =
            (0..3).all(|i| (0..3).all(|j| grid[(r / 3) * 3 + i][(c / 3) * 3 + j] != k));

        not_in_row && not_in_column && not_in_box
    }

    fn solve_recursive(sudoku: &mut Sudoku, r: usize, c: usize) -> Result<(), SolveError> {
        if r == 9 {
            Ok(())
        } else if c == 9 {
            solve_recursive(sudoku, r + 1, 0)
        } else if sudoku.0[r][c] != 0 {
            solve_recursive(sudoku, r, c + 1)
        } else {
            for k in 1..10 {
                if is_valid(&*sudoku, r, c, k) {
                    sudoku.0[r][c] = k;
                    if solve_recursive(sudoku, r, c + 1).is_ok() {
                        return Ok(());
                    }
                    sudoku.0[r][c] = 0;
                }
            }
            Err(SolveError)
        }
    }

    impl fmt::Display for Sudoku {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            for (r, row) in self.0.iter().enumerate() {
                for (c, col) in row.iter().enumerate() {
                    write!(
                        f,
                        "{} {}",
                        col,
                        if (c + 1) % 3 == 0 && c != 8 { " " } else { "" }
                    )?;
                }
                writeln!(f)?;
                if (r + 1) % 3 == 0 && r != 8 {
                    writeln!(f)?;
                }
            }
            Ok(())
        }
    }

    impl FromStr for Sudoku {
        type Err = ParseSudokuError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let string = s.to_string();
            let input = string.trim();
            if input.len() != 9 * 9 {
                Err(ParseSudokuError(format!(
                    "Grid incomplete. {} from {} cells found",
                    input.len(),
                    9 * 9
                )))
            } else {
                let mut grid = [[0; 9]; 9]; // Initialize a 9x9 grid
                let mut char_iter = input.chars();

                for r in 0..9 {
                    for c in 0..9 {
                        let char = char_iter.next().unwrap(); // We already checked input length, so this is safe
                        grid[r][c] = match char {
                            '.' | '0' => 0,
                            '1'..='9' => char.to_digit(10).unwrap() as usize, // Convert character to digit
                            _ => {
                                return Err(ParseSudokuError(
                                    "Input must only contain numbers between 1 and 9 or a '.'"
                                        .to_string(),
                                ))
                            }
                        };
                    }
                }

                Ok(Sudoku(grid))
            }
        }
    }
}
