use anyhow::{Context, Result, Error};
use clap::Parser;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    path: std::path::PathBuf,
}

fn main() -> Result<()> {
    let args = Cli::parse();

    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("could not read file `{}`", args.path.display()))?;

    if args.pattern.is_empty() {
        Err(Error::msg("Pattern must be provided"))
    } else {
        grrs::find_matches(&content, &args.pattern, &mut std::io::stdout());
        Ok(())
    }
}

#[test]
fn find_a_match() {
    let mut results = Vec::new();
    grrs::find_matches("lorem ipsum\ndolor sit amet", "lorem", &mut results);
    assert_eq!(results, b"lorem ipsum\n");
}