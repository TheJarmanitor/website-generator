use std::{env, fs, io, process};

fn main() {
    let Some(path) = env::args().nth(1) else {
        eprintln!("Problem parsing arguments: No path argument was found");
        process::exit(1);
    };
    let post = read_markdown_file(&path).unwrap_or_else(|err| {
        eprintln!("Problem reading file: {err}");
        process::exit(1);
    });

    let post_contents = split_frontmatter(&post);

    println!("{:?}", post_contents);
}

fn read_markdown_file(file_path: &str) -> Result<String, io::Error> {
    let post = fs::read_to_string(file_path)?;
    Ok(post)
}

fn split_frontmatter(content: &str) -> Option<(&str, &str)> {
    if !content.starts_with("---") {
        return None;
    };
    content[3..].trim().split_once("---")
}
