use crate::{files, grid::Grid, lcs};

/// Se representa la operacion de diff
pub struct Diff {
    /// La grilla resultante de aplicar el algoritmos de LCS.
    grid: Grid,
    /// La secuencia original.
    original: Vec<String>,
    /// La secuencia modificada.
    modified: Vec<String>,
}

impl Diff {
    /// Crea un nuevo Diff, a partir de las dos secuencias de String.
    /// # Arguments
    ///
    /// * `path_original` y `path_modified` - Slice de
    /// string que contienen los paths a los archivos a los cuales se
    /// le desea realizar el diff.
    /// # Panics
    ///
    /// Si no se encuentran los archivos ejecuta un panic!()

    pub fn new(path_original: &str, path_modified: &str) -> Self {
        let original = files::read_file_lines(path_original);
        let modified = files::read_file_lines(path_modified);
        Diff {
            grid: lcs::lcs(&original, &modified),
            original,
            modified,
        }
    }

    /// Imprime por stdout el diff entre las dos secuencias.
    pub fn print_diff(&self) {
        print_diff(
            &self.grid,
            &self.original,
            &self.modified,
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
