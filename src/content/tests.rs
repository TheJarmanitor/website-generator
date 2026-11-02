use super::*;

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
