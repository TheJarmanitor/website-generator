use super::*;
use Path;
use std::error::Error;

#[test]
fn frontmatter_return_post_and_meta_split() {
    let mark = "---\ndate: '2026-10-30'\ntitle: 'testing'\n---\nhello world";
    let split = split_frontmatter(mark);

    assert_eq!(
        split,
        Ok(("date: '2026-10-30'\ntitle: 'testing'", "hello world"))
    );
}

#[test]
fn frontmatter_no_opening_delimeter() {
    let mark = "date: '2026-10-30'\ntitle: 'wrong'---";

    let split = split_frontmatter(mark);

    assert_eq!(split.unwrap_err(), FrontMatterError::MissingOpeningLine);
}

#[test]
fn frontmatter_malformed_opening_delimeter() {
    let mark = "----\ndate: '2026-10-30'\ntitle: 'wrong'---";

    let split = split_frontmatter(mark);

    assert_eq!(split.unwrap_err(), FrontMatterError::MissingOpeningLine);
}
#[test]
fn frontmatter_no_break_opening_delimeter() {
    let mark = "---date: '2026-10-30'\ntitle: 'wrong'---";

    let split = split_frontmatter(mark);

    assert_eq!(split.unwrap_err(), FrontMatterError::MissingOpeningLine);
}
#[test]
fn frontmatter_no_closing_delimeter() {
    let mark = "---\ndate: '2026-10-30'\ntitle: 'wrong'";

    let split = split_frontmatter(mark);

    assert_eq!(split.unwrap_err(), FrontMatterError::Unclosed);
}

#[test]
fn frontmatter_extra_delimeter_in_post() {
    let mark = "---\ndate:'2026-10-30\ntitle: 'testing'\n---\nhello\n---\nworld";
    let split = split_frontmatter(mark);

    assert_eq!(
        split,
        Ok(("date:'2026-10-30\ntitle: 'testing'", "hello\n---\nworld"))
    );
}
#[test]
fn frontmatter_extra_delimeter_in_meta() {
    let mark = "---\ndate: '2026-10-30\ntitle: 'wrong---stuff'\n---\nhello world";
    let split = split_frontmatter(mark);

    assert_eq!(
        split,
        Ok(("date: '2026-10-30\ntitle: 'wrong---stuff'", "hello world"))
    );
}
#[test]
fn frontmatter_empty_body() {
    let mark = "---\ndate: '2026-10-30\ntitle: 'empty post'\n---";
    let split = split_frontmatter(mark);

    assert_eq!(split, Ok(("date: '2026-10-30\ntitle: 'empty post'", "")));
}

#[test]
fn frontmatter_empty_yaml() {
    let mark = "---\n---\nbody";
    let split = split_frontmatter(mark);

    assert_eq!(split, Ok(("", "body")));
}

//load_post tests

#[test]
fn load_post_correct_post() {
    let result = load_post(Path::new("tests/fixtures/correct_test.md"))
        .unwrap_or_else(|error| panic!("Expected Ok but got error instead: {error}"));
    assert_eq!(result.post_meta.title.to_string(), "Test");
}

#[test]
fn load_post_no_file() {
    let post = load_post(Path::new("tests/fixtures/nonexistent.md"));

    let Err(error) = post else {
        panic!("Expected an error, but got Ok");
    };

    assert!(
        error
            .source()
            .and_then(|r| r.downcast_ref::<std::io::Error>())
            .is_some()
    );
}

#[test]
fn load_post_no_file_display() {
    let post = load_post(Path::new("tests/fixtures/nonexistent.md"));

    let Err(error) = post else {
        panic!("Expected an error, but got Ok");
    };
    assert!(
        error
            .to_string()
            .starts_with("tests/fixtures/nonexistent.md")
    );
}

#[test]
fn load_post_bad_opening_line() {
    let post = load_post(Path::new("tests/fixtures/bad_opening_line.md"));

    assert!(matches!(
        post,
        Err(ContentError::FrontMatter {
            source: FrontMatterError::MissingOpeningLine,
            path: _
        })
    ));
}

#[test]
fn load_post_bad_opening_line_display() {
    let post = load_post(Path::new("tests/fixtures/bad_opening_line.md"));

    let Err(error) = post else {
        panic!("Expected an error, but got Ok");
    };

    assert_eq!(
        error.to_string(),
        "tests/fixtures/bad_opening_line.md: opening '---' line missing or malformed"
    );
}

#[test]
fn load_post_no_closing_line() {
    let post = load_post(Path::new("tests/fixtures/no_closing_line.md"));

    assert!(matches!(
        post,
        Err(ContentError::FrontMatter {
            source: FrontMatterError::Unclosed,
            path: _
        })
    ));
}

#[test]
fn load_post_no_closing_line_display() {
    let post = load_post(Path::new("tests/fixtures/no_closing_line.md"));

    let Err(error) = post else {
        panic!("Expected an error, but got Ok");
    };

    assert_eq!(
        error.to_string(),
        "tests/fixtures/no_closing_line.md: frontmatter not closed"
    );
}

#[test]
fn load_post_missing_title() {
    let post = load_post(Path::new("tests/fixtures/missing_labels.md"));

    let Err(error) = post else {
        panic!("Expected an error, but got Ok");
    };

    assert!(
        error
            .source()
            .and_then(|r| r.downcast_ref::<yaml_serde::Error>())
            .is_some()
    );
}

#[test]
fn load_post_missing_title_display() {
    let post = load_post(Path::new("tests/fixtures/missing_labels.md"));

    let Err(error) = post else {
        panic!("Expected an error, but got Ok");
    };

    assert!(
        error
            .to_string()
            .starts_with("tests/fixtures/missing_labels.md")
    );
}
