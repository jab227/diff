use ::diffy::diff;
use ::std::{env, io};

fn main() -> Result<(), io::Error> {
    let diff = match diff::Diff::new(env::args()) {
        Ok(d) => d,
        Err(err) => {
            eprintln!("Ocurrio un error: {}", err);
            return Err(err);
        }
    };
    print!("{}", diff);
    Ok(())
}
