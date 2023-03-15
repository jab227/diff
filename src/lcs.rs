use crate::grid;
use itertools::Itertools;
use std::cmp;

/// Algoritmo *Longest Common Subsequence*. Devuelve la grilla
/// generada para ambas secuencias, a partir de la cual se puede
/// recuperar la secuencia comun mas larga.
///
/// # Argumentos
///
/// Toma como argumentos dos secuencias x e y que son slices de Strings.
///
pub fn lcs(x: &[&str], y: &[&str]) -> grid::Grid {
    let mut grid = grid::Grid::new(x.len(), y.len());
    let indexes = (0..x.len(), 0..y.len());
    x.iter()
        .cartesian_product(y.iter())
        .zip(indexes.0.cartesian_product(indexes.1))
        .for_each(|((sx, sy), (i, j))| {
            if sx != sy {
                grid[[i + 1, j + 1]] = cmp::max(grid[[i + 1, j]], grid[[i, j + 1]]);
            } else {
                grid[[i + 1, j + 1]] = grid[[i, j]] + 1;
            }
        });

    grid
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
        let result = lcs(&seqx, &seqy);
        assert_eq!(
            result,
            grid::test_helper::new_grid_from_vec(want, (seqx.len() + 1, seqy.len() + 1))
        );
    }
}
