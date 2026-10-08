use pulldown_cmark::{Options, Parser, TextMergeStream};
use serde::Deserialize;
use std::{env, fs, io, process};

#[derive(Deserialize, Debug)]
struct PostMeta {
    title: String,
    date: String,
    tags: Option<Vec<String>>,
}

fn main() {
    let Some(path) = env::args().nth(1) else {
        eprintln!("Problem parsing arguments: No path argument was found");
        process::exit(1);
    };
    let markdown = read_markdown_file(&path).unwrap_or_else(|err| {
        eprintln!("Problem reading file: {err}");
        process::exit(1);
    });

    let Some((front, post)) = split_frontmatter(&markdown) else {
        eprintln!("Problem reading YAML: no metadata found or broken");
        process::exit(1);
    };
    let meta: PostMeta = yaml_serde::from_str(front).unwrap_or_else(|err| {
        eprintln!("Problem deserializing YAML: {err}");
        process::exit(1);
    });
    let mut options = Options::empty();
    options.insert(Options::ENABLE_MATH);
    let parser = Parser::new_ext(post, options);

    let iterator = TextMergeStream::new(parser).inspect(|x| println!("{:?}", x));

    let mut html_output = String::new();
    pulldown_cmark::html::push_html(&mut html_output, iterator);
    println!("{:?}", meta);
    println!("{}", html_output);
}

fn read_markdown_file(file_path: &str) -> Result<String, io::Error> {
    let post = fs::read_to_string(file_path)?;
    Ok(post)
}

fn split_frontmatter(content: &str) -> Option<(&str, &str)> {
    if !content.starts_with("---") {
        return None;
    };
    content[3..]
        .split_once("---")
        .map(|(m, p)| (m.trim(), p.trim()))
}
