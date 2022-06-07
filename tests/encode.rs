use crate::common::{
    encode_command, gen_not_existing_file, TestResult, EMPTY_FILE, HEADER_ONLY_FILE,
    INVALID_CHUNK_TYPE, MESSAGE, OUTPUT_FILE, VALID_CHUNK_TYPE, VALID_FILE,
};
use assert_cmd::assert::Assert;
use predicates::prelude::predicate;

mod common;

fn encode(filename: &str, chunk_type: &str, message: &str) -> Assert {
    encode_command()
        .arg(filename)
        .arg(chunk_type)
        .arg(message)
        .arg("-o")
        .arg(OUTPUT_FILE)
        .assert()
}

#[test]
fn shows_help() {
    for flag in ["-h", "--help"] {
        encode_command()
            .arg(flag)
            .assert()
            .success()
            .stdout(predicate::str::contains("USAGE:"));
    }
}

#[test]
fn dies_no_args() {
    encode_command()
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "The following required arguments were not provided",
        ));
}

#[test]
fn dies_bad_file() {
    let bad_file = gen_not_existing_file();
    encode(&bad_file, VALID_CHUNK_TYPE, MESSAGE)
        .failure()
        .stderr(predicate::str::contains(
            "The system cannot find the file specified",
        ));
}

#[test]
fn dies_encode_invalid_chunk() {
    encode(EMPTY_FILE, INVALID_CHUNK_TYPE, MESSAGE)
        .failure()
        .stderr(predicate::str::contains("is not a valid png byte"));
}

#[test]
fn encode_valid_to_empty() {
    encode(EMPTY_FILE, VALID_CHUNK_TYPE, MESSAGE).success();
}
