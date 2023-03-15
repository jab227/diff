use ::diffy::diff;
use ::diffy::files;
use ::std::{env, io};

fn main() -> Result<(), io::Error> {
    let (original_path, modified_path) = files::parse_arguments(env::args())?;
    let (original, modified) = files::read_files((&original_path, &modified_path))?;
    let original_ref: Vec<&str> = original.iter().map(|s| s.as_ref()).collect();
    let modified_ref: Vec<&str> = modified.iter().map(|s| s.as_ref()).collect();
    let diff = diff::Diff::new(&original_ref, &modified_ref);
    print!("{}", diff.generate_diff());
    Ok(())
}
