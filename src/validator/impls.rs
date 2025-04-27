use crate::constant;

use super::api::CliArgsValidator;

/// Default implementation of cli args validator.
///
/// Can be used directly as part of [`clap::Arg::value_parser`].
pub struct DefaultCliArgsValidator;

impl CliArgsValidator for DefaultCliArgsValidator {
    /// Checks if given value contains commas.
    ///
    /// Returns [`constant::error_messages::COMMAS_NOT_ALLOWED`] if any commas
    /// found.
    ///
    /// See [`CliArgsValidator`] for more infos.
    fn has_no_commas(value: &str) -> Result<String, String> {
        if value.contains(',') {
            Err(String::from(constant::error_messages::COMMAS_NOT_ALLOWED))
        } else {
            Ok(value.to_string())
        }
    }
}
