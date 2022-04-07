use std::io::{Error, ErrorKind};

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
    pub fn new<I>(mut args: I) -> Result<Self, Error>
    where
        I: ExactSizeIterator<Item = String>,
    {
        if args.len() < 3 {
            return Err(Error::new(
                ErrorKind::Other,
                "Cantidad insuficiente de argumentos",
            ));
        }

        args.next(); // Ignoro el primer argumento
        let original_path = args
            .next()
            .ok_or_else(|| Error::new(ErrorKind::Other, "Ruta invalida"))?;
        let modified_path = args
            .next()
            .ok_or_else(|| Error::new(ErrorKind::Other, "Ruta invalida"))?;

        let original = files::read_file_lines(&original_path)?;
        let modified = files::read_file_lines(&modified_path)?;

        Ok(Diff {
            grid: lcs::lcs(&original, &modified),
            original,
            modified,
        })
    }

    /// Imprime por stdout el diff entre las dos secuencias.
    pub fn print_diff(&self) {
        print!("{}", self.diff_str());
    }

    fn diff_str(&self) -> String {
        let mut s = String::new();
        diff_str(
            &self.grid,
            &self.original,
            &self.modified,
            self.original.len(),
            self.modified.len(),
            &mut s,
        );
        s
    }
}

fn diff_str(grid: &Grid, x: &[String], y: &[String], i: usize, j: usize, s: &mut String) {
    if i > 0 && j > 0 && x[i - 1] == y[j - 1] {
        diff_str(grid, x, y, i - 1, j - 1, s)
    } else if j > 0 && (i == 0 || grid[[i, j - 1]] >= grid[[i - 1, j]]) {
        diff_str(grid, x, y, i, j - 1, s);
        *s = format!("{}> {}\n", s, y[j - 1]);
    } else if i > 0 && (j == 0 || grid[[i, j - 1]] < grid[[i - 1, j]]) {
        diff_str(grid, x, y, i - 1, j, s);
        *s = format!("{}< {}\n", s, x[i - 1]);
    } else {
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn generate_diff_correctly() {
        let files = [
            "".into(),
            "test_lines.txt".into(),
            "test_lines_modified.txt".into(),
        ];
        let diff = Diff::new(files.into_iter()).unwrap();

        let got = diff.diff_str();
        let want = "< bastante conocido y estudiado.\n\
		    < \n\
		    < ejercicio, haremos que calcule la subsecuencia común más larga entre\n\
		    > \n";
        assert_eq!(got, want);
    }
}
