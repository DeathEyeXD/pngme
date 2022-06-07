
use crate::common::command;
use predicates::prelude::*;

mod common;

#[test]
fn correct_bin_name() {
    command();
}

#[test]
fn shows_help() {
    for flag in ["-h", "--help"] {
        command()
            .arg(flag)
            .assert()
            .success()
            .stdout(predicate::str::contains("USAGE:"));
    }
}

#[test]
fn dies_no_args() {
    command()
        .assert()
        .failure()
        .code(2)
        .stderr(predicate::str::contains("USAGE:"));
}
