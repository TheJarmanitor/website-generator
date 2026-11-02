mod error;

pub use error::{ContentError, FrontMatterError};

use pulldown_cmark::{Options, Parser};
use serde::Deserialize;

use std::fs;
use std::path::Path;

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

pub fn load_post(path: &Path) -> Result<Post, ContentError> {
    // io::Error
    let markdown: String = fs::read_to_string(path).map_err(|err| ContentError::Io {
        source: err,
        path: path.to_path_buf(),
    })?;
    // Custom frontmatter error
    let (front, body) = split_frontmatter(&markdown).map_err(|err| ContentError::FrontMatter {
        source: err,
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

fn split_frontmatter(content: &str) -> Result<(&str, &str), FrontMatterError> {
    let stripped_content = content
        .strip_prefix("---")
        .filter(|s| s.starts_with('\n'))
        .ok_or(FrontMatterError::MissingOpeningLine)?;

    stripped_content
        .match_indices("\n---")
        .find_map(|(index, str_match)| {
            let yaml = &stripped_content[..index];
            let rest = &stripped_content[index + str_match.len()..];
            if let Some(body) = rest.strip_prefix('\n') {
                Some((yaml, body))
            } else if rest.is_empty() {
                Some((yaml, ""))
            } else {
                None
            }
        })
        .ok_or(FrontMatterError::Unclosed)
        .map(|(m, p)| (m.trim_start_matches('\n'), p.trim_start_matches('\n')))
}

#[cfg(test)]
mod tests;
