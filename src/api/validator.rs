use crate::api::impls as validator_impl;

pub fn validate_no_commas(template_name: &str) -> Result<String, String> {
    validator_impl::validate_no_commas(template_name)
}
