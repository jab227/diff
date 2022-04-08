use ::diffy::diff;
use std::{process, env};

fn main() {
    let diff = match diff::Diff::new(env::args()) {
        Ok(d) => d,
        Err(err) => {
            eprintln!("Ocurrio un error: {}", err);
            process::exit(1);
        }
    };
    print!("{}",diff);
}
