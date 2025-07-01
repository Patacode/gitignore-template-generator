use std::fs;

use crate::{
    constant::{self, error_messages, path, template_manager},
    printer::{Data, pp},
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
        pp(Data::EnvVarReset(original_value));
        set_env_var(template_manager::HOME_ENV_VAR, original_value);
        pp(Data::Reset());
    }

    fn handle_env_var_removal() {
        pp(Data::EnvVarRemovalAfter());
        remove_env_var(constant::template_manager::HOME_ENV_VAR);
        pp(Data::Removed());
    }
}

impl Drop for EnvTestContext {
    fn drop(&mut self) {
        match &self.original_value {
            Some(original_value) => Self::handle_env_var_reset(original_value),
            None => Self::handle_env_var_removal(),
        }

        pp(Data::TestContextDropped());
    }
}

// fixture
pub fn create_env_test_context() -> EnvTestContext {
    let ctx = match std::env::var(template_manager::HOME_ENV_VAR) {
        Ok(result) => EnvTestContext::new(&result),
        Err(_) => EnvTestContext::empty(),
    };

    pp(Data::TestContextCreated());
    ctx.original_value.is_some().then(handle_env_var_removal);
    ctx
}

fn handle_env_var_removal() {
    pp(Data::EnvVarRemovalBefore());
    remove_env_var(constant::template_manager::HOME_ENV_VAR);
    pp(Data::Removed());
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
    let expectation_file_path = parse_expectation_file_name(expectation_file_name);

    fs::read_to_string(expectation_file_path).expect(error_messages::FILE_READ_TO_STRING_FAILURE)
}

fn parse_expectation_file_name(expectation_file_name: &str) -> String {
    format!(
        "{}/{}/{expectation_file_name}.txt",
        env!("CARGO_MANIFEST_DIR"),
        path::TEST_EXPECTATIONS
    )
}
