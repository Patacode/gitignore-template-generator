use std::{borrow::Cow, fs, process::Output};

use gitignore_template_generator::constant;
use regex::Regex;

pub fn load_expectation_file_as_string(expectation_file_name: &str) -> String {
    let expectation_file_path = format!(
        "{}/{expectation_file_name}.txt",
        constant::path::TEST_EXPECTATIONS
    );

    fs::read_to_string(expectation_file_path)
        .expect(constant::error_messages::FILE_READ_TO_STRING_FAILURE)
}

pub fn get_version_infos_output_pattern() -> Regex {
    Regex::new(&format!(
        r"^{} {}\n$",
        env!("CARGO_PKG_NAME"),
        constant::regex::SEMVER_VERSION,
    ))
    .unwrap()
}

pub fn parse_stdout(stdout: &[u8]) -> Cow<str> {
    String::from_utf8_lossy(stdout)
}
