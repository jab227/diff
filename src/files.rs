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
/// # Panics
///
/// La funcion ejecuta panic!() si ocurre algun error durante
/// la lectura del archivo.
/// TODO: Ver que devuelva result y manejarlo
pub fn read_file_lines(path: &str) -> Vec<String> {
    let fi = match fs::File::open(path) {
        Ok(f) => f,
        Err(error) => panic!("Error abriendo archivo: {:?}", error),
    };
    let reader = io::BufReader::new(fi);
    reader.lines().collect::<io::Result<Vec<String>>>().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_lines_from_file() {
        let path = "test_lines.txt";
        let want = vec![
            "Encontrar la diferencia entre dos archivos es un problema que es",
            "bastante conocido y estudiado.",
            "",
            "La mayoría de las implementaciones usan el algoritmo de Myers, en este",
            "ejercicio, haremos que calcule la subsecuencia común más larga entre",
            "los dos archivos con el algoritmo LCS y use esa información para",
            "calcular su diferencia.",
        ];

        let got = read_file_lines(path);

        assert_eq!(got, want);
    }

    #[test]
    #[should_panic]
    fn panics_when_some_error_happens() {
        let _v = read_file_lines("nonexistent_file.txt");
    }
}
