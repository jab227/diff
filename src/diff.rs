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
    use std::io::Write;

    const PATH_ORIGINAL: &'static str = "original.txt";
    const ORIGINAL_CONTENTS: &'static str = "a\nb\nc\nd\ne";
    const PATH_MODIFIED: &'static str = "modified.txt";
    const MODIFIED_CONTENTS: &'static str = "a\nf\nc\n";

    fn create_tmp_file(path: &str, file_contents: &[u8]) {
        let mut tmp = std::fs::File::create(path).unwrap();
        tmp.write_all(file_contents).unwrap();
    }

    fn del_tmp_file(path: &str) {
        std::fs::remove_file(path).unwrap();
    }

    #[test]
    fn generate_diff_correctly() {
        create_tmp_file(PATH_ORIGINAL, ORIGINAL_CONTENTS.as_bytes());
        create_tmp_file(PATH_MODIFIED, MODIFIED_CONTENTS.as_bytes());

        let files = ["".into(), PATH_ORIGINAL.into(), PATH_MODIFIED.into()];

        let diff = Diff::new(files.into_iter()).unwrap();

        let got = diff.diff_str();
        let want = "< b\n> f\n< d\n< e\n";

        del_tmp_file(PATH_ORIGINAL);
        del_tmp_file(PATH_MODIFIED);

        assert_eq!(got, want);
    }
}
