use std::error::Error;
use std::fmt::Display;
use std::io;
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FrontMatterError {
    MissingOpeningLine,
    Unclosed,
}

impl Display for FrontMatterError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingOpeningLine => f.write_str("opening '---' line missing or malformed"),
            Self::Unclosed => f.write_str("frontmatter not closed"),
        }
    }
}

impl Error for FrontMatterError {}

#[derive(Debug)]
pub enum ContentError {
    Io {
        source: io::Error,
        path: PathBuf,
    },
    FrontMatter {
        source: FrontMatterError,
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
            Self::FrontMatter { source, path } => {
                write!(f, "{}: {source}", path.display())
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
            Self::FrontMatter { source: e, path: _ } => Some(e),
        }
    }
}
