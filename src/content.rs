use pulldown_cmark::{Options, Parser, TextMergeStream};
use serde::Deserialize;
use std::path::Path;
use std::{env, fs, io, process};

#[derive(Deserialize)]
struct PostMeta {
    title: String,
    date: String,
    tags: Option<Vec<String>>,
}

pub struct Post {
    post_meta: PostMeta,
    html_body: String,
}

#[derive(Debug)]
enum ContentError {
    Io(io::Error),
}

pub fn load_post(path: &Path) -> Result<Post, ContentError> {
    let Some(path) = env::args().nth(1) else {
        eprintln!("Problem parsing arguments: No path argument was found");
        process::exit(1);
    };
    let markdown = fs::read_to_string(&path).unwrap_or_else(|err| {
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

    let mut html_output = String::new();
    pulldown_cmark::html::push_html(&mut html_output, parser);
}

fn split_frontmatter(content: &str) -> Option<(&str, &str)> {
    if !content.starts_with("---") {
        return None;
    };
    content[3..]
        .split_once("---")
        .map(|(m, p)| (m.trim(), p.trim()))
}
