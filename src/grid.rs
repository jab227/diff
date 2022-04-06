use std::ops::{Index, IndexMut};

/// Representa la grilla que contiene los largos de las subsecuencias.
///
/// # Indices
///
/// Los indices comienzan en 0, para acceder al elemento en la fila m,
/// columna n se usa la siguiente sintaxis:
///  `grid[[m, n]]`
/// # Ejemplo
///
///
/// ```#s
/// let mut grid = grid::Grid::new(1, 1);
///
/// grid[[0, 0]] = 1;
/// grid[[0, 1]] = 2;
/// grid[[1, 0]] = 3;
/// grid[[1, 1]] = 4;
///
/// assert_eq!(grid[[0,1]],2)
/// ```
/// # Panics
///
/// Se ejecuta panic!() al acceder indices que se encuentren por fuera
/// de la grilla.
///
#[derive(PartialEq, Eq, Debug)]
pub struct Grid {
    /// Grilla unidimensional
    grid: Vec<u32>,
    /// (filas, columnas)
    shape: (usize, usize),
}

impl Grid {
    /// Crea un nuevo Grid con dimensiones (n+1, n+1) y lo inicializa en 0.
    ///
    /// # Ejemplos
    ///
    /// ```#s
    /// use diff::grid;
    ///
    /// let mut grid = grid::Grid::new(1, 1);
    ///
    /// grid[[0, 0]] = 1;
    /// grid[[0, 1]] = 2;
    /// grid[[1, 0]] = 3;
    /// grid[[1, 1]] = 4;
    ///
    /// assert_eq!(grid[[0, 0]], 1);
    /// assert_eq!(grid[[0, 1]], 2);
    /// assert_eq!(grid[[1, 0]], 3);
    /// assert_eq!(grid[[1, 1]], 4);
    /// ```
    pub fn new(m: usize, n: usize) -> Self {
        let grid = vec![0; (m + 1) * (n + 1)];
        let shape = (m + 1, n + 1);
        Grid { grid, shape }
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
pub mod test_helper {
    use crate::grid::Grid;
    pub fn new_grid_from_vec(grid: Vec<u32>, shape: (usize, usize)) -> Grid {
        Grid { grid, shape }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn new_grid() {
        let grid = Grid::new(1, 1);
        assert_eq!(
            grid,
            test_helper::new_grid_from_vec(vec![0, 0, 0, 0], (2, 2))
        );
    }

    #[test]
    fn access_elements_from_grid() {
        let mut grid = Grid::new(1, 1);
        grid[[0, 0]] = 1;
        grid[[0, 1]] = 2;
        grid[[1, 0]] = 3;
        grid[[1, 1]] = 4;

        assert_eq!(
            grid,
            test_helper::new_grid_from_vec(vec![1, 2, 3, 4], (2, 2))
        );

        let new_grid = grid;
        assert_eq!(new_grid[[0, 0]], 1);
    }

    #[test]
    #[should_panic]
    fn panics_when_out_of_bounds_indexes() {
        let mut grid = Grid::new(1, 1);
        grid[[0, 0]] = 1;
        grid[[0, 1]] = 2;
        grid[[1, 0]] = 3;
        grid[[1, 1]] = 4;

        let out_of_bounds = grid[[2, 2]];
        println!("{}", out_of_bounds);
    }
}
