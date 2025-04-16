mod impls;
pub mod validator;

use crate::impls as api_impl;

pub trait HttpClient {
    fn get(&self, url: &str) -> Result<String, String>;
}

pub fn get_call_to_gitignore_template_service(values: &String) -> String {
    api_impl::get_call_to_gitignore_template_service(values)
}
