use url::Url;

use super::api::CliArgsValidator;
use crate::constant;

/// Default implementation of cli args validator.
///
/// Can be used directly as part of [`clap::Arg::value_parser`].
pub struct DefaultCliArgsValidator;

impl DefaultCliArgsValidator {
    fn has_valid_scheme(url: &Url) -> bool {
        url.scheme() == "http" || url.scheme() == "https"
    }
}

impl CliArgsValidator for DefaultCliArgsValidator {
    /// Checks if given value does not contain any commas.
    ///
    /// Returns [`constant::error_messages::COMMAS_NOT_ALLOWED`] if any commas
    /// found.
    ///
    /// See [`CliArgsValidator::has_no_commas`] for more infos.
    fn has_no_commas(value: &str) -> Result<String, String> {
        match (!value.contains(',')).then(|| value.to_string()) {
            Some(value) => Ok(value),
            None => Err(constant::error_messages::COMMAS_NOT_ALLOWED.to_string()),
        }
    }

    /// Checks if given value does not contain any whitespaces.
    ///
    /// Returns [`constant::error_messages::WHITESPACES_NOT_ALLOWED`] if any
    /// whitespaces found.
    ///
    /// See [`CliArgsValidator::has_no_whitespaces`] for more infos.
    fn has_no_whitespaces(value: &str) -> Result<String, String> {
        match value
            .chars()
            .all(|c| !c.is_whitespace())
            .then(|| value.to_string())
        {
            Some(value) => Ok(value),
            None => Err(constant::error_messages::WHITESPACES_NOT_ALLOWED.to_string()),
        }
    }

    fn is_valid_template_name(value: &str) -> Result<String, String> {
        Self::has_no_commas(value).and_then(|value| Self::has_no_whitespaces(&value))
    }

    fn is_starting_with_slash(value: &str) -> Result<String, String> {
        match value.starts_with('/').then(|| value.to_string()) {
            Some(value) => Ok(value),
            None => Err(constant::error_messages::URI_WITHOUT_STARTING_SLASH.to_string()),
        }
    }

    fn is_valid_url(value: &str) -> Result<String, String> {
        match Url::parse(value) {
            Ok(url) if Self::has_valid_scheme(&url) => Ok(value.to_string()),
            Ok(_) => Err(constant::error_messages::INVALID_SCHEME.to_string()),
            Err(_) => Err(constant::error_messages::INVALID_URL.to_string()),
        }
    }
}
