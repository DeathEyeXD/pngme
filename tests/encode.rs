use assert_cmd::assert::Assert;
use predicates::prelude::predicate;

use pngme::get_png;

use crate::common::{
    encode_command, gen_not_existing_file, EMPTY_FILE,
    INVALID_CHUNK_TYPE, INVALID_HEADER_FILE, MESSAGE, OUTPUT_FILE, VALID_CHUNK_TYPE, VALID_FILE,
};

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

fn has_last_message(message: &str) {
    let png = get_png(OUTPUT_FILE).unwrap();
    assert_eq!(
        message,
        png.chunks().last().unwrap().data_as_string().unwrap()
    );
}

fn encode_empty(filename: &str) {
    encode(filename, VALID_CHUNK_TYPE, "abc").success();

    has_last_message("abc")
}
fn encode_invalid(filename: &str) {
    encode(filename, INVALID_CHUNK_TYPE, MESSAGE)
        .failure()
        .stderr(predicates::str::contains("is not a valid png byte"));
}

fn encode_valid(filename: &str) {
    encode(filename, VALID_CHUNK_TYPE, MESSAGE).success();

    has_last_message(MESSAGE)
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
fn dies_encode_valid_to_empty() {
    encode(EMPTY_FILE, VALID_CHUNK_TYPE, MESSAGE)
        .failure()
        .stderr(predicates::str::contains(
            "Valid png must have at least 8 bytes (missing header) but only 0 were provided",
        ));
}
#[test]
fn encode_valid_to_valid() {
    encode_valid(VALID_FILE);
}

#[test]
fn dies_encode_invalid_to_valid() {
    encode_invalid(VALID_FILE);
}

#[test]
fn encode_empty_to_valid() {
    encode_empty(VALID_FILE);
}

#[test]
fn dies_encode_valid_to_invalid_header() {
    encode(INVALID_HEADER_FILE, VALID_CHUNK_TYPE, MESSAGE)
        .failure()
        .stderr(predicates::str::contains(
            "Valid png must contain valid signature header",
        ));
}
