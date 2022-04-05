use grid::Grid;

pub mod files;
pub mod grid;
pub mod lcs;

/// Se representa la operacion de diff
pub struct Diff<'a> {
    /// La grilla resultante de aplicar el algoritmos de LCS.
    grid: Grid,
    /// La secuencia original.
    original: &'a [String],
    /// La secuencia modificada.
    modified: &'a [String],
}

impl<'a> Diff<'a> {
    /// Crea un nuevo Diff, a partir de las dos secuencias de String.
    /// # Arguments
    /// * `original` y `modified` - Slice de strings entre
    /// los cuales se realiza el diff.
    pub fn new(original: &'a [String], modified: &'a [String]) -> Self {
        Diff {
            grid: lcs::lcs(original, modified),
            original,
            modified,
        }
    }

    /// Imprime por stdout el diff entre las dos secuencias.
    pub fn print_diff(&self) {
        print_diff(
            &self.grid,
            self.original,
            self.modified,
            self.original.len(),
            self.modified.len(),
        );
    }
}

/// Funcion auxiliar que imprime por stdout el diff entre las dos
/// secuencias de manera recursiva.
fn print_diff(grid: &Grid, x: &[String], y: &[String], i: usize, j: usize) {
    if i > 0 && j > 0 && x[i - 1] == y[j - 1] {
        print_diff(grid, x, y, i - 1, j - 1);
    } else if j > 0 && (i == 0 || grid[[i, j - 1]] >= grid[[i - 1, j]]) {
        print_diff(grid, x, y, i, j - 1);
        println!("> {}", y[j - 1]);
    } else if i > 0 && (j == 0 || grid[[i, j - 1]] < grid[[i - 1, j]]) {
        print_diff(grid, x, y, i - 1, j);
        println!("< {}", x[i - 1]);
    } else {
        println!();
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
