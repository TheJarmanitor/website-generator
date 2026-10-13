use pulldown_cmark::{Options, Parser};
use serde::Deserialize;
use std::error::Error;
use std::fmt::Display;
use std::path::{Path, PathBuf};
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

#[derive(Debug)]
pub enum ContentError {
    Io {
        source: io::Error,
        path: PathBuf,
    },
    FrontMatter {
        path: PathBuf,
    },
    Yaml {
        source: yaml_serde::Error,
        path: PathBuf,
    },
}

impl Display for ContentError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io { source, path } => write!(f, "{}: {source}", path.display()),
            Self::FrontMatter { path } => {
                write!(
                    f,
                    "{}: frontmatter not found or broken in file ",
                    path.display()
                )
            }
            Self::Yaml { source, path } => {
                write!(f, "{}: {source}", path.display())
            }
        }
    }
}
impl Error for ContentError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Io { source: e, path: _ } => Some(e),
            Self::Yaml { source: e, path: _ } => Some(e),
            Self::FrontMatter { .. } => None,
        }
    }
}

pub fn load_post(path: &Path) -> Result<Post, ContentError> {
    // io::Error
    let markdown: String = fs::read_to_string(path).map_err(|err| ContentError::Io {
        source: err,
        path: path.to_path_buf(),
    })?;
    // Custom frontmatter error
    let (front, body) = split_frontmatter(&markdown).ok_or_else(|| ContentError::FrontMatter {
        path: path.to_path_buf(),
    })?;
    // yaml_serde::Error
    let meta: PostMeta = yaml_serde::from_str(front).map_err(|err| ContentError::Yaml {
        source: err,
        path: path.to_path_buf(),
    })?;
    let mut options = Options::empty();
    options.insert(Options::ENABLE_MATH);
    let parser = Parser::new_ext(body, options);

    let mut html_output = String::new();
    pulldown_cmark::html::push_html(&mut html_output, parser);

    let post = Post {
        post_meta: meta,
        html_body: html_output,
    };

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
