use std::{borrow::Cow, fs};

use gitignore_template_generator::constant;

pub fn load_expectation_file_as_string(expectation_file_name: &str) -> String {
    let expectation_file_path = format!(
        "{}/{expectation_file_name}.txt",
        constant::path::TEST_EXPECTATIONS
    );

    fs::read_to_string(expectation_file_path)
        .expect(constant::error_messages::FILE_READ_TO_STRING_FAILURE)
}

pub fn parse_bytes(bytes: &[u8]) -> Cow<str> {
    String::from_utf8_lossy(bytes)
}

pub fn parse_pos_args(pos_args: &str) -> Vec<&str> {
    pos_args.split_whitespace().collect()
}

pub fn get_help_message() -> String {
    load_expectation_file_as_string("help_message")
        .replace("{pkg_name}", env!("CARGO_PKG_NAME"))
        .replace("{about}", constant::parser_infos::ABOUT)
        .replace(
            "{template_names_desc}",
            constant::help_messages::TEMPLATE_NAMES,
        )
        .replace("{author_desc}", constant::help_messages::AUTHOR)
        .replace("{server_url_desc}", constant::help_messages::SERVER_URL)
        .replace("{help_desc}", constant::help_messages::HELP)
        .replace("{version_desc}", constant::help_messages::VERSION)
        .replace("{version}", env!("CARGO_PKG_VERSION"))
        .replace("{author}", env!("CARGO_PKG_AUTHORS"))
        .replace(
            "{author_short}",
            constant::cli_options::AUTHOR.short.to_string().as_str(),
        )
        .replace("{author_long}", constant::cli_options::AUTHOR.long)
        .replace(
            "{server_url_short}",
            constant::cli_options::SERVER_URL.short.to_string().as_str(),
        )
        .replace("{server_url_long}", constant::cli_options::SERVER_URL.long)
        .replace(
            "{help_short}",
            constant::cli_options::HELP.short.to_string().as_str(),
        )
        .replace("{help_long}", constant::cli_options::HELP.long)
        .replace(
            "{version_short}",
            constant::cli_options::VERSION.short.to_string().as_str(),
        )
        .replace("{version_long}", constant::cli_options::VERSION.long)
        .replace(
            "{server_url_default}",
            constant::template_generator::BASE_URL,
        )
}
