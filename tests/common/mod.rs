use assert_cmd::Command;
use rand::distributions::Alphanumeric;
use rand::Rng;
use std::fs;

pub type TestResult = pngme::Result<()>;

const BIN_NAME: &str = "pngme";

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
