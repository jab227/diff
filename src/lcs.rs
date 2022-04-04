use std::cmp;

use crate::grid;
fn lcs(x: &str, y: &str) -> grid::Grid {
    let mut grid = grid::Grid::new(x.len(), y.len());
    println!("{:?}", grid);
    for (i, cx) in x.chars().enumerate() {
        for (j, cy) in y.chars().enumerate() {
            if cx == cy {
                grid[[i + 1, j + 1]] = grid[[i, j]] + 1;
            } else {
                grid[[i + 1, j + 1]] = cmp::max(grid[[i + 1, j]], grid[[i, j + 1]]);
            }
        }
    }

    grid
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_lcs() {
        let seqx = "acbcf";
        let seqy = "abcdaf";

        let want = vec![
            0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 0, 1, 1, 2, 2, 2, 2, 0, 1, 2, 2, 2, 2, 2, 0,
            1, 2, 3, 3, 3, 3, 0, 1, 2, 3, 3, 3, 4,
        ];
        let result = lcs(seqx, seqy);
        assert_eq!(result, grid::Grid::new_from_vec(want, result.shape()));
    }
}
