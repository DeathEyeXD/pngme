use assert_cmd::prelude::*;

use crate::common::{command, TestResult};
use predicates::prelude::*;

mod common;

#[test]
fn correct_bin_name() -> TestResult {
    command();

    Ok(())
}

#[test]
fn shows_help() -> TestResult {
    for flag in ["-h", "--help"] {
        command()
            .arg(flag)
            .assert()
            .success()
            .stdout(predicate::str::contains("USAGE:"));
    }
    Ok(())
}

#[test]
fn dies_no_args() -> TestResult {
    command()
        .assert()
        .failure()
        .code(2)
        .stderr(predicate::str::contains("USAGE:"));
    Ok(())
}
