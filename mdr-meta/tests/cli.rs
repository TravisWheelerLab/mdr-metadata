use anyhow::Result;
use assert_cmd::Command;
use predicates::prelude::*;
//use pretty_assertions::assert_eq;
use rand::distr::{Alphanumeric, SampleString};
use std::fs;

const PRG: &str = "mdr-meta";
const TRUNCATED_TOML: &str = "../tests/inputs/truncated.toml";
const TRUNCATED_JSON: &str = "../tests/inputs/truncated.json";
const EMPTY: &str = "../tests/inputs/empty";
const EMPTY_TOML: &str = "../tests/inputs/empty.toml";
const EMPTY_JSON: &str = "../tests/inputs/empty.json";

// --------------------------------------------------
fn gen_bad_file() -> String {
    loop {
        let filename =
            format!("{}.toml", Alphanumeric.sample_string(&mut rand::rng(), 7));

        if fs::metadata(&filename).is_err() {
            return filename;
        }
    }
}

// --------------------------------------------------
#[test]
fn dies_no_args() -> Result<()> {
    Command::cargo_bin(PRG)?
        .assert()
        .failure()
        .stderr(predicate::str::contains("Usage"));
    Ok(())
}

// --------------------------------------------------
#[test]
fn dies_bad_file() -> Result<()> {
    let bad = gen_bad_file();
    Command::cargo_bin(PRG)?
        .args(["check", &bad])
        .assert()
        .stderr(predicate::str::is_match(" No such file or directory")?);
    Ok(())
}

// --------------------------------------------------
#[test]
fn dies_no_file_extension() -> Result<()> {
    Command::cargo_bin(PRG)?
        .args(&["check", EMPTY])
        .assert()
        .failure()
        .stderr(predicate::str::contains("No file extension"));
    Ok(())
}

// --------------------------------------------------
#[test]
fn dies_empty_json() -> Result<()> {
    Command::cargo_bin(PRG)?
        .args(&["check", EMPTY_JSON])
        .assert()
        .failure()
        .stderr(predicate::str::contains("File is empty"));
    Ok(())
}

// --------------------------------------------------
#[test]
fn dies_empty_toml() -> Result<()> {
    Command::cargo_bin(PRG)?
        .args(&["check", EMPTY_TOML])
        .assert()
        .failure()
        .stderr(predicate::str::contains("File is empty"));
    Ok(())
}

// --------------------------------------------------
#[test]
fn dies_trucated_toml() -> Result<()> {
    Command::cargo_bin(PRG)?
        .args(&["check", TRUNCATED_TOML])
        .assert()
        .failure()
        .stderr(predicate::str::contains("TOML parse error"));
    Ok(())
}

// --------------------------------------------------
#[test]
fn dies_truncated_json() -> Result<()> {
    Command::cargo_bin(PRG)?
        .args(&["check", TRUNCATED_JSON])
        .assert()
        .failure()
        .stderr(predicate::str::contains("EOF while parsing a string"));
    Ok(())
}
