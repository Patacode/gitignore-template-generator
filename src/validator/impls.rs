use super::api::CliArgsValidator;

pub struct DefaultCliArgsValidator;

impl CliArgsValidator for DefaultCliArgsValidator {
    fn has_no_commas(value: &str) -> Result<String, String> {
        if value.contains(',') {
            Err(String::from("Commas are not allowed in file names."))
        } else {
            Ok(value.to_string())
        }
    }
}
