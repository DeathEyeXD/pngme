use std::fs;

use assert_cmd::assert::Assert;

use crate::common::{
    remove_command, DIFFERENT_MESSAGE, EMPTY_FILE, MESSAGE, VALID_CHUNK_TYPE,
    VALID_DIFFERENT_CHUNK_TYPE, VALID_ENCODED1, VALID_ENCODED2, VALID_ENCODED2_DIFFERENT,
    VALID_FILE, TestResult
};
const TEMP_FILE: &str = "tests/temp.png";
mod common;
fn remove(filename: &str, chunk: &str) -> Assert {
    remove_command().arg(filename).arg(chunk).assert()
}

fn remove_existing(filename: &str, chunk_type: &str, message: &str, copy_to_temp: bool) -> TestResult {
    if copy_to_temp { 
        fs::copy(filename, TEMP_FILE)?;
    }
    remove(filename, chunk_type)
        .success()
        .stdout(predicates::str::contains(format!(
            "deleted chunk with message '{}'",
            message
        )));
    Ok(())
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
fn remove_valid() -> TestResult {
    remove_existing(VALID_ENCODED1, VALID_CHUNK_TYPE, MESSAGE,true)
}

#[test]
fn remove_2_valid() -> TestResult {
    remove_existing(VALID_ENCODED2_DIFFERENT, VALID_CHUNK_TYPE, MESSAGE,true)?;
    remove_existing(
        TEMP_FILE,
        VALID_DIFFERENT_CHUNK_TYPE,
        DIFFERENT_MESSAGE,false
    )
}

#[test]
fn dies_remove_invalid() {
    remove_not_existing(VALID_FILE, VALID_CHUNK_TYPE);
}

#[test]
fn remove_first_from_two_valid() -> TestResult{
    remove_existing(VALID_ENCODED2, VALID_CHUNK_TYPE, MESSAGE,true)
}
#[test]
fn dies_remove_from_empty() {
    remove(EMPTY_FILE, VALID_CHUNK_TYPE)
        .failure()
        .stderr(predicates::str::contains("Invalid png file data"));
}
