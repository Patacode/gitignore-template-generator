use crate::validator::impls;

pub fn validate_no_commas(template_name: &str) -> Result<String, String> {
    impls::validate_no_commas(template_name)
}