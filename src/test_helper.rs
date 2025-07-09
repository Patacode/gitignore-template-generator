use std::ffi::OsString;

mod impls;

#[cfg(test)]
mod tests;

pub struct EnvTestContext {
    original_value: Option<String>,
}

pub trait TestUtils {
    fn create_env_test_context() -> EnvTestContext;
    fn remove_env_var<T: AsRef<std::ffi::OsStr>>(name: T);
    fn set_env_var<T: AsRef<std::ffi::OsStr>, V: AsRef<std::ffi::OsStr>>(name: T, value: V);
    fn load_expectation_file(expectation_file_name: &str) -> String;
    fn get_expectation_file_path(expectation_file_name: &str) -> String;
    fn load_resource_file(resource_file_name: &str) -> String;
    fn get_resource_file_path(resource_name: &str) -> String;
    fn parse_cli_args(cli_args: &str) -> Vec<&str>;
    fn parse_and_map_cli_args<B, F>(cli_args: &str, mapper: F) -> Vec<B>
    where
        F: FnMut(&str) -> B;
    fn to_os_string(value: &str) -> OsString;
    fn to_string_list(values: &str) -> Vec<String>;
    fn get_help_message() -> String;
    fn get_ansi_help_message() -> String;
}

pub struct DefaultTestUtils;
