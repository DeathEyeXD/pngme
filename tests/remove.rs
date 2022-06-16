use crate::common::{
    remove_command, DIFFERENT_MESSAGE, EMPTY_FILE, MESSAGE, SECOND_MESSAGE, VALID_CHUNK_TYPE,
    VALID_DIFFERENT_CHUNK_TYPE, VALID_ENCODED1, VALID_ENCODED2, VALID_ENCODED2_DIFFERENT,
    VALID_FILE,
};
use assert_cmd::assert::Assert;
use pngme::Chunk;

mod common;
fn remove(filename: &str, chunk: &str) -> Assert {
    remove_command().arg(filename).arg(chunk).assert()
}

fn remove_existing(filename: &str, chunk_type: &str, message: &str) {
    remove(filename, chunk_type)
        .success()
        .stdout(predicates::str::contains(format!(
            "deleted chunk with message '{}'",
            message
        )));
}

fn remove_not_existing(filename: &str, chunk_type: &str) {
    remove(filename, chunk_type)
        .failure()
        .stderr(predicates::str::contains(format!(
            "No chunk with type '{}' was found",
            chunk_type
        )));
}

#[test]
fn remove_valid() {
    remove_existing(VALID_ENCODED1, VALID_CHUNK_TYPE, MESSAGE);
}

#[test]
fn remove_2_valid() {
    remove_existing(VALID_ENCODED2_DIFFERENT, VALID_CHUNK_TYPE, MESSAGE);
    remove_existing(
        VALID_ENCODED2_DIFFERENT,
        VALID_DIFFERENT_CHUNK_TYPE,
        DIFFERENT_MESSAGE,
    );
}

#[test]
fn dies_remove_invalid() {
    remove_not_existing(VALID_FILE, VALID_CHUNK_TYPE);
}

#[test]
fn remove_first_from_two_valid() {
    remove_existing(VALID_ENCODED2, VALID_CHUNK_TYPE, MESSAGE);
}
#[test]
fn dies_remove_from_empty() {
    remove(EMPTY_FILE, VALID_CHUNK_TYPE)
        .failure()
        .stderr(predicates::str::contains("Invalid png file data"));
}
