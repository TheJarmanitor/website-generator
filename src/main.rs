mod content;
use std::path::Path;
use std::{env, process};

fn main() {
    let string_path = env::args().nth(1).unwrap_or_else(|| {
        eprintln!("Argument missing. Please provide at least one");
        process::exit(1);
    });
    let post = content::load_post(Path::new(&string_path));

    match post {
        Ok(p) => println!("post body working:\n {}", p.html_body),
        Err(e) => {
            eprintln!("error: {e}");
            process::exit(1);
        }
    }
}
