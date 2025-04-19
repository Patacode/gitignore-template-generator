use std::{borrow::Cow, fs};

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
    let pattern = format!(
        r"^{} {}\n$",
        env!("CARGO_PKG_NAME"),
        constant::regex::SEMVER_VERSION,
    );

    Regex::new(&pattern).unwrap()
}

pub fn parse_stdout(stdout: &[u8]) -> Cow<str> {
    String::from_utf8_lossy(stdout)
}

pub fn parse_pos_args(pos_args: &str) -> Vec<&str> {
    pos_args.split_whitespace().collect()
}

pub fn get_regex_no_match_error_message(
    actual_output: &str,
    expected_pattern: &Regex,
) -> String {
    constant::error_messages::REGEX_NO_MATCH
        .replace("{actual}", actual_output)
        .replace("{expected}", expected_pattern.as_str())
}
