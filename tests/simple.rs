
use assert_cmd::prelude::*;
use std::process::Command;

#[test]
fn no_args() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("tq")?;
    cmd.assert().failure().stderr(
        predicates::str::contains("Missing Argument"));
    Ok(())
}
#[test]
fn no_file() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("tq")?;
    cmd.arg(".");
    cmd.assert().failure().stderr(
        predicates::str::contains("No toml to parse"));
    Ok(())
}

#[test]
fn get_int() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("tq")?;
    cmd.arg(".table.int");
    cmd.arg("tests/example1.toml");
    cmd.assert().success().stdout(
        predicates::str::contains("2"));
    Ok(())
}
#[test]
fn get_float() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("tq")?;
    cmd.arg(".table.float");
    cmd.arg("tests/example1.toml");
    cmd.assert().success().stdout(
        predicates::str::contains("5.8"));
    Ok(())
}
#[test]
fn get_str() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("tq")?;
    cmd.arg(".table.str");
    cmd.arg("tests/example1.toml");
    cmd.assert().success().stdout(
        predicates::str::contains("testing"));
    Ok(())
}
#[test]
fn get_bool() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("tq")?;
    cmd.arg(".table.bool");
    cmd.arg("tests/example1.toml");
    cmd.assert().success().stdout(
        predicates::str::contains("true"));
    Ok(())
}
#[test]
fn get_date() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("tq")?;
    cmd.arg(".table.date");
    cmd.arg("tests/example1.toml");
    cmd.assert().success().stdout(
        predicates::str::contains("2021-06-15"));
    Ok(())
}
#[test]
fn get_array() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("tq")?;
    cmd.arg(".table.array");
    cmd.arg("tests/example1.toml");
    cmd.assert().success().stdout(
        predicates::str::is_match(r"[\s*1,\s*2,\s*3,\s*4\s*]").unwrap());
    Ok(())
}
