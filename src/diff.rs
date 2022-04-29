//! Se implementa la funcionalidad del diff entre dos archivos
use std::{
    fmt::Display,
    io::{Error, ErrorKind},
};

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
    ///
    /// # Argumentos
    ///
    /// * `args` es un iterador sobre strings que debe cumplir con el
    /// trait `ExactSizeIterator`. Debe ser de largo 3, sino devuelve
    /// un error; el primer elemento se ignora.
    ///
    /// # Errores
    ///
    /// Si falla en la lectura de los argumentos o en la lectura de
    /// los archivos devuelve un error del tipo `std::io::Error`.
    ///
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

    /// Metodo recursivo que genera un string que contiene el diff
    /// entre los dos archivos.
    ///
    ///# Argumentos
    ///
    /// * i y j se inicializan con la longitud de las secuencias sobre
    /// las que se realizara el diff
    /// * s se debe inicializar con el string vacio.
    ///
    fn diff_str(&self, i: usize, j: usize, s: &mut String) {
        if i > 0 && j > 0 && self.original[i - 1] == self.modified[j - 1] {
            self.diff_str(i - 1, j - 1, s)
        } else if j > 0 && (i == 0 || self.grid[[i, j - 1]] >= self.grid[[i - 1, j]]) {
            self.diff_str(i, j - 1, s);
            *s = format!("{}> {}\n", s, self.modified[j - 1]);
        } else if i > 0 && (j == 0 || self.grid[[i, j - 1]] < self.grid[[i - 1, j]]) {
            self.diff_str(i - 1, j, s);
            *s = format!("{}< {}\n", s, self.original[i - 1]);
        } else {
        }
    }
}

impl Display for Diff {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        self.diff_str(self.original.len(), self.modified.len(), &mut s);
        write!(f, "{}", s)
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

    fn delete_tmp_file(path: &str) {
                       std::fs::remove_file(path).unwrap();
    }

    #[test]
    fn generate_diff_correctly() {
        create_tmp_file(PATH_ORIGINAL, ORIGINAL_CONTENTS.as_bytes());
         create_tmp_file(PATH_MODIFIED, MODIFIED_CONTENTS.as_bytes());

        let files = ["".into(), PATH_ORIGINAL.into(), PATH_MODIFIED.into()];

              let diff = Diff::new(files.into_iter()).unwrap();

        let got = format!("{}", diff);
               let want = "< b\n> f\n< d\n< e\n";

        delete_tmp_file(PATH_ORIGINAL);
        delete_tmp_file(PATH_MODIFIED);

        assert_eq!(got, want);
    }
}
