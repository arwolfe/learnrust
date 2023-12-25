#[derive(Debug)]
pub struct CustomError(String);


pub fn find_matches(
    content: &str,
    pattern: &str,
    mut writer: impl std::io::Write,
) {
    for line in content.lines() {
        if line.contains(pattern) {
            let _ = writeln!(writer, "{}", line)
                .map_err(|err| CustomError(format!("Error writing `{}`: {}", line, err)));
        }
    }
}
