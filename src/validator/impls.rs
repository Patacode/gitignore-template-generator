use crate::constant;

use super::api::CliArgsValidator;

pub struct DefaultCliArgsValidator;

impl CliArgsValidator for DefaultCliArgsValidator {
    fn has_no_commas(value: &str) -> Result<String, String> {
        if value.contains(',') {
            Err(String::from(constant::error_messages::COMMAS_NOT_ALLOWED))
        } else {
            Ok(value.to_string())
        }
    }
}
