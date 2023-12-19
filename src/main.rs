use anyhow::{Context, Result};
use clap::Parser;
use log::{info, warn};

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    path: std::path::PathBuf,
}

/*
use std::{io::{BufReader, BufRead}, fs::File, path};

fn main01() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();
    let file = File::open(&args.path).expect("could not read file");
    let mut final_content = String::new();
    let buffer = BufReader::new(file);

    for line in buffer.lines() {
        match line {
            Ok(content) if content.contains(&args.pattern) => {
                final_content = content;
                break
            },
            Ok(_) => println!("Not a match."),
            Err(error) => {
                return Err(error.into())
            },
        };
    }

    println!("Match -> [{final_content}]");

    Ok(())
}

#[derive(Debug)]
struct CustomError(String);

fn main02() -> Result<(), CustomError> {
    let path = "test.txt";
    let content = std::fs::read_to_string(path)
        .map_err(|err| CustomError(format!("Error reading `{}`: {}", path, err)))?;
    println!("file content: {}", content);
    Ok(())
}


*/

#[derive(Debug)]
struct CustomError(String);

fn main() -> Result<()> {
    let args = Cli::parse();

    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("could not read file `{}`", args.path.display()))?;

    find_matches(&content, &args.pattern, &mut std::io::stdout());

    Ok(())
}

#[test]
fn find_a_match() {
    let mut results = Vec::new();
    find_matches("lorem ipsum\ndolor sit amet", "lorem", &mut results);
    assert_eq!(results, b"lorem ipsum\n");
}

fn find_matches(
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

// fn main() {
//     env_logger::init();

//     info!("starting up");
//     warn!("oops, nothing implemented!");
//     let pb = indicatif::ProgressBar::new(10);
//     for i in 0..10 {
//         log::info!("working...");
//         do_hard_work();
//         pb.println(format!("[+] finished #{}", i));
//         pb.inc(1);
//     }
//     pb.finish_with_message("done");
// }

fn do_hard_work() {
    std::thread::sleep(std::time::Duration::from_millis(500));
}

fn answer() -> i32 {
    42
}

#[test]
fn check_answer_validity() {
    assert_eq!(answer(), 42);
}

// https://rust-cli.github.io/book/tutorial/testing.html