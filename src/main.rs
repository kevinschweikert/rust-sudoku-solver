use std::fmt;

type Grid = [[usize; 9]; 9];
struct GridWrapper(Grid);

impl fmt::Display for GridWrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for r in 0..9 {
            for c in 0..9 {
                write!(f, "{} ", self.0[r][c])?;
                if (c + 1) % 3 == 0 {
                    write!(f, " ")?;
                }
            }
            writeln!(f)?;
            if (r + 1) % 3 == 0 {
                writeln!(f)?;
            }
        }
        writeln!(f)
    }
}

fn main() {
    let mut grid: Grid = [
        [0, 0, 0, 4, 0, 6, 0, 3, 5],
        [0, 6, 0, 0, 0, 0, 0, 0, 0],
        [5, 0, 2, 3, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 4, 1, 0],
        [3, 0, 0, 0, 9, 0, 0, 7, 0],
        [4, 9, 0, 0, 0, 0, 8, 2, 0],
        [0, 0, 0, 2, 0, 9, 0, 0, 0],
        [0, 0, 8, 7, 1, 0, 0, 0, 0],
        [0, 7, 0, 0, 0, 0, 3, 0, 1],
    ];

    solve(&mut grid, 0, 0);
    println!("{}", GridWrapper(grid));
}

fn is_valid(grid: &mut Grid, r: usize, c: usize, k: usize) -> bool {
    let not_in_row = !grid[r].contains(&k);
    let not_in_column = (0..8).all(|i| grid[i][c] != k);

    let r_start = ((r as f64 / 3.0).floor() * 3.0).round() as usize;
    let r_end = r_start + 3;
    let c_start = ((c as f64 / 3.0).floor() * 3.0).round() as usize;
    let c_end = c_start + 3;

    let not_in_box = !grid[r_start..r_end]
        .iter()
        .any(|cols| cols[c_start..c_end].iter().any(|i| *i == k));
    not_in_row && not_in_column && not_in_box
}

fn solve(grid: &mut Grid, r: usize, c: usize) -> bool {
    if r == 9 {
        true
    } else if c == 9 {
        solve(grid, r + 1, 0)
    } else if grid[r][c] != 0 {
        solve(grid, r, c + 1)
    } else {
        for k in 1..10 {
            if is_valid(grid, r, c, k) {
                grid[r][c] = k;
                if solve(grid, r, c + 1) {
                    return true;
                }
                grid[r][c] = 0;
            }
        }
        false
    }
}

#[test]
fn is_valid_test() {
    let mut grid: Grid = [
        [0, 0, 4, 0, 5, 0, 0, 0, 0],
        [9, 5, 0, 7, 3, 4, 6, 0, 0],
        [0, 0, 3, 0, 2, 1, 0, 4, 9],
        [0, 3, 5, 0, 9, 0, 4, 8, 0],
        [0, 9, 0, 0, 0, 0, 0, 3, 0],
        [0, 7, 6, 0, 1, 0, 9, 2, 0],
        [3, 1, 0, 9, 7, 0, 2, 0, 0],
        [0, 0, 9, 1, 8, 2, 0, 0, 3],
        [0, 0, 0, 0, 6, 0, 1, 0, 0],
    ];

    assert!(is_valid(&mut grid, 0, 0, 1));
    assert!(!is_valid(&mut grid, 0, 0, 4));
    assert!(!is_valid(&mut grid, 0, 0, 3));
    assert!(!is_valid(&mut grid, 0, 0, 5));
}
