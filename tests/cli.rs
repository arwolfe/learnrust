use anyhow::Error;
use assert_cmd::prelude::*; // Add methods on commands
use predicates::prelude::*; // Used for writing assertions
use std::process::Command; // Run programs

#[test]
fn file_doesnt_exist() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("grrs")?;

    cmd.arg("foobar").arg("test/file/doesnt/exist");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("could not read file"));

    Ok(())
}

use assert_fs::{prelude::*, NamedTempFile};

fn make_temp_file() -> Result<NamedTempFile,Error> {
    let file = assert_fs::NamedTempFile::new("sample.txt")?;
    file.write_str("A test\nActual content\nMore content\nAnother test")?;

    Ok(file)
}

#[test]
fn find_content_in_file() -> Result<(), Box<dyn std::error::Error>> {
    let _test_file = make_temp_file().unwrap();

    let mut cmd = Command::cargo_bin("grrs")?;
    cmd.arg("test").arg(_test_file.path());
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("A test\nAnother test"));

    Ok(())
}

#[test]
fn empty_pattern() -> Result<(), Box<dyn std::error::Error>> {
    let _test_file = make_temp_file().unwrap();

    let mut cmd = Command::cargo_bin("grrs")?;

    cmd.arg("").arg(_test_file.path()).unwrap();
    cmd.assert()
        .stderr(predicate::eq(b"Pattern must be provided" as &[u8]));

    Ok(())
}