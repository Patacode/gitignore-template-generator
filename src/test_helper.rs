use std::{ffi::OsString, fs};

use crate::{
    constant::{
        cli_options, error_messages, help_messages, help_texts, parser_infos, path,
        template_manager,
    },
    printer::{Data, pp, ppg},
};

pub struct EnvTestContext {
    original_value: Option<String>,
}

impl EnvTestContext {
    fn new(original_value: &str) -> Self {
        Self {
            original_value: Some(original_value.to_string()),
        }
    }

    fn empty() -> Self {
        Self {
            original_value: None,
        }
    }

    fn handle_env_var_reset(original_value: &str) {
        pp(&Data::EnvVarReset(original_value));
        set_env_var(template_manager::HOME_ENV_VAR, original_value);
        pp(&Data::Any(help_texts::RESET));
    }

    fn handle_env_var_removal() {
        pp(&Data::EnvVarRemovalAfter());
        remove_env_var(template_manager::HOME_ENV_VAR);
        pp(&Data::Any(help_texts::REMOVED));
    }
}

impl Drop for EnvTestContext {
    fn drop(&mut self) {
        match &self.original_value {
            Some(original_value) => Self::handle_env_var_reset(original_value),
            None => Self::handle_env_var_removal(),
        }

        pp(&Data::Any(help_texts::TEST_CXT_DROPPED));
    }
}

// fixture
pub fn create_env_test_context() -> EnvTestContext {
    let ctx = match std::env::var(template_manager::HOME_ENV_VAR) {
        Ok(result) => EnvTestContext::new(&result),
        Err(_) => EnvTestContext::empty(),
    };

    pp(&Data::Any(help_texts::TEST_CTX_CREATED));
    ctx.original_value.is_some().then(handle_env_var_removal);
    ctx
}

fn handle_env_var_removal() {
    pp(&Data::EnvVarRemovalBefore());
    remove_env_var(template_manager::HOME_ENV_VAR);
    pp(&Data::Any(help_texts::REMOVED));
}

pub fn remove_env_var<T: AsRef<std::ffi::OsStr>>(name: T) {
    unsafe {
        std::env::remove_var(name);
    }
}

pub fn set_env_var<T: AsRef<std::ffi::OsStr>, V: AsRef<std::ffi::OsStr>>(name: T, value: V) {
    unsafe {
        std::env::set_var(name, value);
    }
}

pub fn load_expectation_file(expectation_file_name: &str) -> String {
    let expect_filepath = get_expectation_file_path(expectation_file_name);

    fs::read_to_string(expect_filepath).expect(error_messages::FILE_READ_TO_STRING_FAILURE)
}

pub fn load_resource_file(resource_file_name: &str) -> String {
    let res_filepath = get_resource_file_path(resource_file_name);

    fs::read_to_string(res_filepath).expect(error_messages::FILE_READ_TO_STRING_FAILURE)
}

pub fn get_expectation_file_path(expectation_file_name: &str) -> String {
    format!(
        "{}/{}/{expectation_file_name}.txt",
        env!("CARGO_MANIFEST_DIR"),
        path::TEST_EXPECTATIONS
    )
}

pub fn get_resource_file_path(resource_name: &str) -> String {
    format!(
        "{}/{}/{resource_name}",
        env!("CARGO_MANIFEST_DIR"),
        path::TEST_RESOURCES
    )
}

pub fn parse_cli_args(cli_args: &str) -> Vec<&str> {
    cli_args.split_whitespace().collect()
}

pub fn parse_and_map_cli_args<B, F>(cli_args: &str, mapper: F) -> Vec<B>
where
    F: FnMut(&str) -> B,
{
    format!("{} {cli_args}", env!("CARGO_PKG_NAME"))
        .split_whitespace()
        .map(mapper)
        .collect()
}

pub fn to_os_string(value: &str) -> OsString {
    OsString::from(value)
}

pub fn to_string_list(values: &str) -> Vec<String> {
    values.split_whitespace().map(String::from).collect()
}

pub fn get_help_message() -> String {
    parse_expectation_file_to_help_message("help_message")
}

pub fn get_ansi_help_message() -> String {
    parse_expectation_file_to_help_message("ansi_help_message")
}

fn parse_expectation_file_to_help_message(template_name: &str) -> String {
    load_expectation_file(template_name)
        .replace("{pkg_name}", env!("CARGO_PKG_NAME"))
        .replace("{about}", parser_infos::ABOUT)
        .replace("{template_names_desc}", help_messages::TEMPLATE_NAMES)
        .replace("{author_desc}", help_messages::AUTHOR)
        .replace("{server_url_desc}", help_messages::SERVER_URL)
        .replace("{help_desc}", help_messages::HELP)
        .replace("{version_desc}", help_messages::VERSION)
        .replace("{version}", env!("CARGO_PKG_VERSION"))
        .replace("{author}", env!("CARGO_PKG_AUTHORS"))
        .replace("{author_short}", cli_options::AUTHOR.short)
        .replace("{author_long}", cli_options::AUTHOR.long)
        .replace("{server_url_short}", cli_options::SERVER_URL.short)
        .replace("{server_url_long}", cli_options::SERVER_URL.long)
        .replace("{help_short}", cli_options::HELP.short)
        .replace("{help_long}", cli_options::HELP.long)
        .replace("{version_short}", cli_options::VERSION.short)
        .replace("{version_long}", cli_options::VERSION.long)
        .replace("{server_url_default}", template_manager::BASE_URL)
        .replace("{generator_uri_short}", cli_options::GENERATOR_URI.short)
        .replace("{generator_uri_long}", cli_options::GENERATOR_URI.long)
        .replace("{generator_uri_desc}", help_messages::GENERATOR_URI)
        .replace("{generator_uri_default}", template_manager::GENERATOR_URI)
        .replace("{list_short}", cli_options::LIST.short)
        .replace("{list_long}", cli_options::LIST.long)
        .replace("{list_desc}", help_messages::LIST)
        .replace("{lister_uri_short}", cli_options::LISTER_URI.short)
        .replace("{lister_uri_long}", cli_options::LISTER_URI.long)
        .replace("{lister_uri_desc}", help_messages::LISTER_URI)
        .replace("{lister_uri_default}", template_manager::LISTER_URI)
        .replace("{check_short}", cli_options::CHECK.short)
        .replace("{check_long}", cli_options::CHECK.long)
        .replace("{check_desc}", help_messages::CHECK)
        .replace("{timeout_short}", cli_options::TIMEOUT.short)
        .replace("{timeout_long}", cli_options::TIMEOUT.long)
        .replace("{timeout_desc}", help_messages::TIMEOUT)
        .replace("{timeout_default}", &ppg(&Data::DefaultTimeout()))
        .replace("{timeout_unit_short}", cli_options::TIMEOUT_UNIT.short)
        .replace("{timeout_unit_long}", cli_options::TIMEOUT_UNIT.long)
        .replace("{timeout_unit_desc}", help_messages::TIMEOUT_UNIT)
        .replace("{timeout_unit_default}", template_manager::TIMEOUT_UNIT)
        .replace("{timeout_unit_values}", "millisecond, second")
}
