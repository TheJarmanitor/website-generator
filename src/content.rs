use pulldown_cmark::{Options, Parser};
use serde::Deserialize;
use std::fmt::Display;
use std::path::Path;
use std::{fs, io};

#[derive(Deserialize)]
struct PostMeta {
    title: String,
    date: String,
    tags: Option<Vec<String>>,
}

pub struct Post {
    post_meta: PostMeta,
    pub html_body: String,
}

impl Post {
    fn new(post_meta: PostMeta, html_body: String) -> Self {
        Post {
            post_meta,
            html_body,
        }
    }
}
#[derive(Debug)]
pub enum ContentError {
    IoError(io::Error),
    FrontMatterError(),
    YamlError(yaml_serde::Error),
}

impl Display for ContentError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ContentError::IoError(io_error) => write!(f, "{}", io_error),
            ContentError::FrontMatterError()
        }
    }
}

pub fn load_post(path: &Path) -> Result<Post, ContentError> {
    // io::Error
    let markdown: String = fs::read_to_string(&path)?;
    // Custom frontmatter error
    let (front, body) = split_frontmatter(&markdown).ok_or(ContentError::Front)?;
    // yaml_serde::Error
    let meta: PostMeta = yaml_serde::from_str(front)?;
    let mut options = Options::empty();
    options.insert(Options::ENABLE_MATH);
    let parser = Parser::new_ext(body, options);

    let mut html_output = String::new();
    pulldown_cmark::html::push_html(&mut html_output, parser);

    let post = Post::new(meta, html_output);

    Ok(post)
}

fn split_frontmatter(content: &str) -> Result<(&str, &str), ContentError> {
    if !content.starts_with("---") {
        return;
    };
    content[3..]
        .split_once("---")
        .map(|(m, p)| (m.trim(), p.trim()))
}
