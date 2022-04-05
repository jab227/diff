use std::ops::{Index, IndexMut};

/// Representa la grilla que contiene los largos de las subsecuencias.
#[derive(PartialEq, Eq, Debug)]
pub struct Grid {
    /// Grilla unidimensional
    grid: Vec<u32>,
    /// (filas, columnas)
    shape: (usize, usize),
}

impl Grid {
    /// .
    ///
    /// # Examples
    ///
    /// ```
    /// use diff::grid::Grid;
    ///
    /// assert_eq!(Grid::new(m, n), );
    /// ```
    pub fn new(m: usize, n: usize) -> Self {
        let grid = vec![0; (m + 1) * (n + 1)];
        let shape = (m + 1, n + 1);
        Grid { grid, shape }
    }

    /// .
    ///
    /// # Examples
    ///
    /// ```
    /// use diff::grid::Grid;
    ///
    /// assert_eq!(Grid::new(m, n), );
    pub fn new_from_vec(grid: Vec<u32>, shape: (usize, usize)) -> Self {
        Grid { grid, shape }
    }

    /// .
    ///
    /// # Examples
    ///
    /// ```
    /// use diff::grid::Grid;
    ///
    /// let grid = ;
    /// assert_eq!(grid.shape(), );
    /// ```
    pub fn shape(&self) -> (usize, usize) {
        self.shape
    }
}

impl Index<[usize; 2]> for Grid {
    type Output = u32;

    fn index(&self, index: [usize; 2]) -> &Self::Output {
        let width = self.shape.1;
        if index[0] >= self.shape.0 || index[1] >= self.shape.1 {
            panic!(
                "index out of bounds: the indexes were ({},{}), but the shape of the grid is ({}, {})",
                index[0], index[1], self.shape.0, self.shape.1
            )
        }

        &self.grid[width * index[0] + index[1]]
    }
}

impl IndexMut<[usize; 2]> for Grid {
    fn index_mut(&mut self, index: [usize; 2]) -> &mut Self::Output {
        let width = self.shape.1;
        if index[0] >= self.shape.0 || index[1] >= self.shape.1 {
            panic!(
                "index out of bounds: the indexes were ({},{}), but the shape of the grid is ({}, {})",
                index[0], index[1], self.shape.0, self.shape.1
            )
        }
        &mut self.grid[width * index[0] + index[1]]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn new_grid() {
        let grid = Grid::new(1, 1);
        assert_eq!(grid, Grid::new_from_vec(vec![0, 0, 0, 0], (2, 2)));
    }

    #[test]
    fn access_elements_from_grid() {
        let mut grid = Grid::new(1, 1);
        grid[[0, 0]] = 1;
        grid[[0, 1]] = 2;
        grid[[1, 0]] = 3;
        grid[[1, 1]] = 4;

        assert_eq!(grid, Grid::new_from_vec(vec![1, 2, 3, 4], (2, 2)));

        let new_grid = grid;
        assert_eq!(new_grid[[0, 0]], 1);
    }
}
