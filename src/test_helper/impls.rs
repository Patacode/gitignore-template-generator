use std::{ffi::OsString, fs};

use crate::{
    constant::{
        cli_options, error_messages, help_messages, help_texts, parser_infos, path,
        template_manager,
    },
    printer::{Data, DataPrinter, DefaultDataPrinter},
    test_helper::{DefaultTestUtils, EnvTestContext, TestUtils},
};

impl TestUtils for DefaultTestUtils {
    // fixture
    fn create_env_test_context() -> EnvTestContext {
        let ctx = match std::env::var(template_manager::HOME_ENV_VAR) {
            Ok(result) => EnvTestContext::new(&result),
            Err(_) => EnvTestContext::empty(),
        };

        DefaultDataPrinter::pp(&Data::Any(help_texts::TEST_CTX_CREATED));
        ctx.original_value.is_some().then(handle_env_var_removal);
        ctx
    }

    fn remove_env_var<T: AsRef<std::ffi::OsStr>>(name: T) {
        unsafe {
            std::env::remove_var(name);
        }
    }

    fn set_env_var<T: AsRef<std::ffi::OsStr>, V: AsRef<std::ffi::OsStr>>(name: T, value: V) {
        unsafe {
            std::env::set_var(name, value);
        }
    }

    fn load_expectation_file(expectation_file_name: &str) -> String {
        let expect_filepath = Self::get_expectation_file_path(expectation_file_name);

        fs::read_to_string(expect_filepath).expect(error_messages::FILE_READ_TO_STRING_FAILURE)
    }

    fn load_resource_file(resource_file_name: &str) -> String {
        let res_filepath = Self::get_resource_file_path(resource_file_name);

        fs::read_to_string(res_filepath).expect(error_messages::FILE_READ_TO_STRING_FAILURE)
    }

    fn get_expectation_file_path(expectation_file_name: &str) -> String {
        format!(
            "{}/{}/{expectation_file_name}.txt",
            env!("CARGO_MANIFEST_DIR"),
            path::TEST_EXPECTATIONS
        )
    }

    fn get_resource_file_path(resource_name: &str) -> String {
        format!(
            "{}/{}/{resource_name}",
            env!("CARGO_MANIFEST_DIR"),
            path::TEST_RESOURCES
        )
    }

    fn parse_cli_args(cli_args: &str) -> Vec<&str> {
        cli_args.split_whitespace().collect()
    }

    fn parse_and_map_cli_args<B, F>(cli_args: &str, mapper: F) -> Vec<B>
    where
        F: FnMut(&str) -> B,
    {
        format!("{} {cli_args}", env!("CARGO_PKG_NAME"))
            .split_whitespace()
            .map(mapper)
            .collect()
    }

    fn to_os_string(value: &str) -> OsString {
        OsString::from(value)
    }

    fn to_string_list(values: &str) -> Vec<String> {
        values.split_whitespace().map(String::from).collect()
    }

    fn get_help_message() -> String {
        parse_expectation_file_to_help_message("help_message")
    }

    fn get_ansi_help_message() -> String {
        parse_expectation_file_to_help_message("ansi_help_message")
    }
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
        DefaultDataPrinter::pp(&Data::EnvVarReset(original_value));
        DefaultTestUtils::set_env_var(template_manager::HOME_ENV_VAR, original_value);
        DefaultDataPrinter::pp(&Data::Any(help_texts::RESET));
    }

    fn handle_env_var_removal() {
        DefaultDataPrinter::pp(&Data::EnvVarRemovalAfter());
        DefaultTestUtils::remove_env_var(template_manager::HOME_ENV_VAR);
        DefaultDataPrinter::pp(&Data::Any(help_texts::REMOVED));
    }
}

impl Drop for EnvTestContext {
    fn drop(&mut self) {
        match &self.original_value {
            Some(original_value) => Self::handle_env_var_reset(original_value),
            None => Self::handle_env_var_removal(),
        }

        DefaultDataPrinter::pp(&Data::Any(help_texts::TEST_CXT_DROPPED));
    }
}

fn handle_env_var_removal() {
    DefaultDataPrinter::pp(&Data::EnvVarRemovalBefore());
    DefaultTestUtils::remove_env_var(template_manager::HOME_ENV_VAR);
    DefaultDataPrinter::pp(&Data::Any(help_texts::REMOVED));
}

fn parse_expectation_file_to_help_message(template_name: &str) -> String {
    DefaultTestUtils::load_expectation_file(template_name)
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
        .replace(
            "{timeout_default}",
            &DefaultDataPrinter::ppg(&Data::DefaultTimeout()),
        )
        .replace("{timeout_unit_short}", cli_options::TIMEOUT_UNIT.short)
        .replace("{timeout_unit_long}", cli_options::TIMEOUT_UNIT.long)
        .replace("{timeout_unit_desc}", help_messages::TIMEOUT_UNIT)
        .replace("{timeout_unit_default}", template_manager::TIMEOUT_UNIT)
        .replace("{timeout_unit_values}", "millisecond, second")
}
