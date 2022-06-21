use std::{fs};

use assert_cmd::Command;
use rand::distributions::Alphanumeric;
use rand::Rng;

pub type TestResult = pngme::Result<()>;

const BIN_NAME: &str = "pngme";

pub const VALID_CHUNK_TYPE: &str = "ruSt";
pub const VALID_DIFFERENT_CHUNK_TYPE: &str = "tyPe";
pub const INVALID_CHUNK_TYPE: &str = "1234";

pub const MESSAGE: &str = "top secret message";

pub const SECOND_MESSAGE: &str = "second message";
pub const DIFFERENT_MESSAGE: &str = "message with different type";

pub const EMPTY_FILE: &str = "tests/inputs/empty.png";
pub const HEADER_ONLY_FILE: &str = "tests/inputs/header_only.png";
pub const INVALID_HEADER_FILE: &str = "tests/inputs/invalid_header.png";
pub const VALID_FILE: &str = "tests/inputs/valid.png";

pub const VALID_ENCODED1: &str = "tests/inputs/valid_encoded1.png";
pub const VALID_ENCODED2: &str = "tests/inputs/valid_encoded2.png";
pub const VALID_ENCODED2_DIFFERENT: &str = "tests/inputs/valid_encoded2_different.png";

pub const OUTPUT_FILE: &str = "tests/test.png";

#[test]
fn test_files_exist() {
    for file in [
        EMPTY_FILE,
        HEADER_ONLY_FILE,
        INVALID_HEADER_FILE,
        VALID_FILE,
        OUTPUT_FILE,
    ] {
        assert!(fs::metadata(file).is_ok(), "file {} does not exist", file);
    }
}

pub fn command() -> Command {
    Command::cargo_bin(BIN_NAME).unwrap()
}

pub fn gen_not_existing_file() -> String {
    loop {
        let filename: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(8)
            .map(char::from)
            .collect();

        if fs::metadata(&filename).is_err() {
            return filename;
        }
    }
}

pub fn encode_command() -> Command {
    let mut command = command();
    command.arg("encode");

    command
}
pub fn decode_command() -> Command {
    let mut command = command();
    command.arg("decode");

    command
}
pub fn remove_command() -> Command {
    let mut command = command();

    command.arg("remove");

    command
}
