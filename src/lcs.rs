use crate::grid;
use std::cmp;

pub fn lcs(x: &Vec<String>, y: &Vec<String>) -> grid::Grid {
    let mut grid = grid::Grid::new(x.len(), y.len());
    for (i, sx) in x.iter().enumerate() {
        for (j, sy) in y.iter().enumerate() {
            if sx == sy {
                grid[[i + 1, j + 1]] = grid[[i, j]] + 1;
            } else {
                grid[[i + 1, j + 1]] = cmp::max(grid[[i + 1, j]], grid[[i, j + 1]]);
            }
        }
    }
    grid
}

pub fn print_diff(grid: grid::Grid, x: &Vec<String>, y: &Vec<String>, i: usize, j: usize) {
    if i > 0 && j > 0 && x[i - 1] == y[j - 1] {
        print_diff(grid, x, y, i - 1, j - 1);
    } else if j > 0 && (i == 0 || grid[[i, j - 1]] >= grid[[i - 1, j]]) {
        print_diff(grid, x, y, i, j - 1);
        println!("> {}", y[j - 1]);
    } else if i > 0 && (j == 0 || grid[[i, j - 1]] < grid[[i - 1, j]]) {
        print_diff(grid, x, y, i - 1, j);
        println!("< {}", x[i - 1]);
    } else {
        println!("");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_lcs() {
        let seqx = vec!["a".into(), "c".into(), "b".into(), "c".into(), "f".into()];
        let seqy = vec![
            "a".into(),
            "b".into(),
            "c".into(),
            "d".into(),
            "a".into(),
            "f".into(),
        ];

        let want = vec![
            0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 0, 1, 1, 2, 2, 2, 2, 0, 1, 2, 2, 2, 2, 2, 0,
            1, 2, 3, 3, 3, 3, 0, 1, 2, 3, 3, 3, 4,
        ];
        let result = lcs(seqx, seqy);
        assert_eq!(result, grid::Grid::new_from_vec(want, result.shape()));
    }
}
