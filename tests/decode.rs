use assert_cmd::assert::Assert;

use crate::common::{
    decode_command, DIFFERENT_MESSAGE, EMPTY_FILE, INVALID_HEADER_FILE, MESSAGE, SECOND_MESSAGE,
    VALID_CHUNK_TYPE, VALID_DIFFERENT_CHUNK_TYPE, VALID_ENCODED1, VALID_ENCODED2,
    VALID_ENCODED2_DIFFERENT, VALID_FILE,
};

mod common;

fn decode(filename: &str, chunk_type: &str) -> Assert {
    decode_command().arg(filename).arg(chunk_type).assert()
}

fn decode_message(filename: &str, chunk_type: &str, expected_message: &str) {
    decode(filename, chunk_type)
        .success()
        .stdout(predicates::str::contains(format!(
            "secret message: '{}'",
            expected_message
        )));
}

fn assert_no_message(filename: &str, chunk_type: &str) {
    decode(filename, chunk_type)
        .success()
        .stdout(predicates::str::contains(format!(
            "No chunk with type '{}' was found",
            chunk_type
        )));
}

#[test]
fn decode_valid1() {
    decode_message(VALID_ENCODED1, VALID_CHUNK_TYPE, MESSAGE);
}

#[test]
fn decode_valid2_last() {
    decode_message(VALID_ENCODED2, VALID_CHUNK_TYPE, SECOND_MESSAGE);
    // decode_message(VALID_ENCODED2, VALID_CHUNK_TYPE, MESSAGE);
}

#[test]
fn decode_valid2_different_last() {
    decode_message(
        VALID_ENCODED2_DIFFERENT,
        VALID_DIFFERENT_CHUNK_TYPE,
        DIFFERENT_MESSAGE,
    );
    // decode_message(VALID_ENCODED2_DIFFERENT, VALID_CHUNK_TYPE, MESSAGE);
}

#[test]
fn dies_decode_empty() {
    decode(EMPTY_FILE, VALID_CHUNK_TYPE)
        .failure()
        .stderr(predicates::str::contains(
            "Invalid png file data (Valid png must have at least 8 bytes (missing header) but only 0 were provided",
        ));
}

#[test]
fn dies_decode_invalid() {
    decode(INVALID_HEADER_FILE, VALID_CHUNK_TYPE)
        .failure()
        .stderr(predicates::str::contains(
            "Valid png must contain valid signature header",
        ));
}

#[test]
fn decode_nothing_from_valid() {
    let chunk_type = VALID_CHUNK_TYPE;
    decode(VALID_FILE, chunk_type)
        .success()
        .stdout(predicates::str::contains(format!(
            "No chunk with type '{}' was found",
            chunk_type
        )));
}
