mod impls;
pub mod validator;

use crate::impls as api_impl;

#[derive(Clone)]
pub struct ProgramError {
    pub message: String,
    pub exit_status: i32,
}

pub trait HttpClient {
    fn get(&self, url: &str) -> Result<String, ProgramError>;
}

pub fn get_call_to_gitignore_template_service(values: &String) -> String {
    api_impl::get_call_to_gitignore_template_service(values)
}
