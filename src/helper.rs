//! Define components to help in other modules.
//!
//! Generic place to put helper code.

use std::ffi::OsString;

use crate::{constant, core::QualifiedString, test_helper::load_expectation_file};

pub fn parse_pos_args(pos_args: &str) -> Vec<&str> {
    pos_args.split_whitespace().collect()
}

pub fn parse_cli_args(pos_args: &str) -> Vec<OsString> {
    format!("{} {pos_args}", env!("CARGO_PKG_NAME"))
        .split_whitespace()
        .map(OsString::from)
        .collect()
}

pub fn make_string_vec(values: &str) -> Vec<String> {
    values.split_whitespace().map(String::from).collect()
}

pub fn get_help_message() -> String {
    get_help_message_for("help_message")
}

pub fn get_ansi_help_message() -> String {
    get_help_message_for("ansi_help_message")
}

pub fn get_help_message_for(template_name: &str) -> String {
    load_expectation_file(template_name)
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
        .replace("{server_url_default}", constant::template_manager::BASE_URL)
        .replace(
            "{generator_uri_short}",
            constant::cli_options::GENERATOR_URI
                .short
                .to_string()
                .as_str(),
        )
        .replace(
            "{generator_uri_long}",
            constant::cli_options::GENERATOR_URI.long,
        )
        .replace(
            "{generator_uri_desc}",
            constant::help_messages::GENERATOR_URI,
        )
        .replace(
            "{generator_uri_default}",
            constant::template_manager::GENERATOR_URI,
        )
        .replace(
            "{list_short}",
            constant::cli_options::LIST.short.to_string().as_str(),
        )
        .replace("{list_long}", constant::cli_options::LIST.long)
        .replace("{list_desc}", constant::help_messages::LIST)
        .replace(
            "{lister_uri_short}",
            constant::cli_options::LISTER_URI.short.to_string().as_str(),
        )
        .replace("{lister_uri_long}", constant::cli_options::LISTER_URI.long)
        .replace("{lister_uri_desc}", constant::help_messages::LISTER_URI)
        .replace(
            "{lister_uri_default}",
            constant::template_manager::LISTER_URI,
        )
        .replace(
            "{check_short}",
            constant::cli_options::CHECK.short.to_string().as_str(),
        )
        .replace("{check_long}", constant::cli_options::CHECK.long)
        .replace("{check_desc}", constant::help_messages::CHECK)
        .replace(
            "{timeout_short}",
            constant::cli_options::TIMEOUT.short.to_string().as_str(),
        )
        .replace("{timeout_long}", constant::cli_options::TIMEOUT.long)
        .replace("{timeout_desc}", constant::help_messages::TIMEOUT)
        .replace(
            "{timeout_default}",
            format!(
                "{}s/{}ms",
                constant::template_manager::TIMEOUT,
                constant::template_manager::TIMEOUT_MILLISECOND
            )
            .as_str(),
        )
        .replace(
            "{timeout_unit_short}",
            constant::cli_options::TIMEOUT_UNIT
                .short
                .to_string()
                .as_str(),
        )
        .replace(
            "{timeout_unit_long}",
            constant::cli_options::TIMEOUT_UNIT.long,
        )
        .replace("{timeout_unit_desc}", constant::help_messages::TIMEOUT_UNIT)
        .replace(
            "{timeout_unit_default}",
            constant::template_manager::TIMEOUT_UNIT,
        )
        .replace("{timeout_unit_values}", "millisecond, second")
}

pub fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + c.as_str(),
    }
}

pub fn insert_at(l: &mut Vec<QualifiedString>, idx: usize, val: QualifiedString) {
    let mut tail = l.split_off(idx);
    l.push(val);
    l.append(&mut tail);
}
