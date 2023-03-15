//! Se implementa la funcionalidad del diff entre dos archivos

use crate::{grid::Grid, lcs};

/// Se representa la operacion de diff
pub struct Diff<'a> {
    /// La grilla resultante de aplicar el algoritmos de LCS.
    grid: Grid,
    /// La secuencia original.
    original: &'a [&'a str],
    /// La secuencia modificada.
    modified: &'a [&'a str],
}

impl<'a> Diff<'a> {
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
    pub fn new(original: &'a [&'a str], modified: &'a [&'a str]) -> Self {
        Diff {
            grid: lcs::lcs(original, modified),
            original,
            modified,
        }
    }

    pub(crate) fn grid(&self) -> &Grid {
        &self.grid
    }

    pub(crate) fn contents(&self) -> (&'a [&'a str], &'a [&'a str]) {
        (self.original, self.modified)
    }

    pub fn generate_diff(&self) -> String {
        //        self.diff_str_iter()
        let i = self.original.len() as isize;
        let j = self.modified.len() as isize;
        let it = DiffIterator { diff: self, i, j };

        let report = it.fold(DiffReport::builder(), |acc, c| match c {
            Changes::Added(s) => acc.added(s),
            Changes::Changed(s) => acc.changed(s),
            Changes::Unchanged => acc,
        });
        report.build()
    }
}

struct DiffIterator<'a> {
    pub diff: &'a Diff<'a>,
    pub i: isize,
    pub j: isize,
}

enum Changes<'a> {
    Added(&'a str),
    Changed(&'a str),
    Unchanged,
}

impl<'a> Iterator for DiffIterator<'a> {
    type Item = Changes<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if !(self.i > 0 || self.j > 0) {
            return None;
        }
        let i = self.i as usize;
        let j = self.j as usize;
        let (original, modified) = self.diff.contents();
        let grid = self.diff.grid();

        if i == 0 {
            self.j -= 1;
            return Some(Changes::Added(modified[j - 1]));
        } else if j == 0 {
            self.i -= 1;
            return Some(Changes::Changed(modified[i - 1]));
        } else if original[i - 1] == modified[j - 1] {
            self.i -= 1;
            self.j -= 1;
            return Some(Changes::Unchanged);
        } else if grid[[i, j - 1]] >= grid[[i - 1, j]] {
            self.j -= 1;
            return Some(Changes::Added(modified[j - 1]));
        } else {
            self.i -= 1;
            return Some(Changes::Changed(original[i - 1]));
        }
    }
}

struct DiffReport(String);

impl DiffReport {
    pub fn builder() -> Self {
        Self(String::new())
    }

    pub fn changed(mut self, s: &str) -> Self {
        self.0.push('\n');
        self.0.push_str(s);
        self.0.push_str(" <");
        self
    }

    pub fn added(mut self, s: &str) -> Self {
        self.0.push('\n');
        self.0.push_str(s);
        self.0.push_str(" >");
        self
    }

    pub fn build(self) -> String {
        self.0.chars().rev().collect()
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_diff_correctly() {
        let original_contents: Vec<&str> = vec!["a", "b", "c", "d", "e"];
        let modified_contents: Vec<&str> = vec!["a", "f", "c"];
        let diff = Diff::new(&original_contents, &modified_contents);

        let got = diff.generate_diff();
        let want = "< b\n> f\n< d\n< e\n";

        assert_eq!(got, want);
    }
}
