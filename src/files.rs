use std::{
    fs,
    io::{self, BufRead},
};

/// Lee las lineas del archivo de texto especificado en path  y devuelve un vector
/// de strings con las lineas del archivo.
///
/// # Ejemplos
/// ```#s
/// let want = vec![
///          "Encontrar la diferencia entre dos archivos es un problema que es",
///          "bastante conocido y estudiado.",
///          "",
///          "La mayoría de las implementaciones usan el algoritmo de Myers, en este",
///          "ejercicio, haremos que calcule la subsecuencia común más larga entre",
///          "los dos archivos con el algoritmo LCS y use esa información para",
///          "calcular su diferencia.",
///     ];
///
/// use crate::files;
/// let path = "test_lines.txt";
///
/// assert_eq!(read_file_lines(path), want);
/// ```
///
/// # Errores
///
/// La funcion devuelve un Result<Vec<String>, std::io::Error>>
///
pub fn read_file_lines(path: &str) -> io::Result<Vec<String>> {
    let fi = fs::File::open(path)?;
    let reader = io::BufReader::new(fi);
    reader.lines().collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    const PATH: &'static str = "tmp.txt";
    const TEST_STRING: &'static str = "a.\nb.\nc.\nd.\ne,";

    fn create_tmp_file(path: &str, file_contents: &[u8]) {
        let mut tmp = fs::File::create(path).unwrap();
        tmp.write_all(file_contents).unwrap();
    }

    fn del_tmp_file(path: &str) {
        fs::remove_file(path).unwrap();
    }

    #[test]
    fn reads_lines_from_file() {
        create_tmp_file(PATH, TEST_STRING.as_bytes());
        let want = vec!["a.", "b.","c.","d.","e,"];

        let got = read_file_lines(PATH);

        del_tmp_file(PATH);

        assert_eq!(got.unwrap(), want);
    }
}
