mod content;

fn main() {}

fn read_markdown_file(file_path: &str) -> Result<String, io::Error> {
    let post = fs::read_to_string(file_path)?;
    Ok(post)
}
