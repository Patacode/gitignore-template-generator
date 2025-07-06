use url::Url;

use super::api::CliArgsValidator;
use crate::constant;

/// Default implementation of cli args validator.
///
/// Can be used directly as part of [`clap::Arg::value_parser`].
pub struct DefaultCliArgsValidator;

impl CliArgsValidator for DefaultCliArgsValidator {
    /// Checks if given value does not contain any commas.
    ///
    /// Returns [`constant::error_messages::COMMAS_NOT_ALLOWED`] if any commas
    /// found.
    ///
    /// See [`CliArgsValidator::has_no_commas`] for more infos.
    fn has_no_commas(value: &str) -> Result<String, String> {
        if value.contains(',') {
            Err(String::from(constant::error_messages::COMMAS_NOT_ALLOWED))
        } else {
            Ok(value.to_string())
        }
    }

    /// Checks if given value contains whitespaces.
    ///
    /// Returns [`constant::error_messages::WHITESPACES_NOT_ALLOWED`] if any
    /// whitespaces found.
    ///
    /// See [`CliArgsValidator::has_no_whitespaces`] for more infos.
    fn has_no_whitespaces(value: &str) -> Result<String, String> {
        if value.chars().any(|c| c.is_whitespace()) {
            Err(String::from(
                constant::error_messages::WHITESPACES_NOT_ALLOWED,
            ))
        } else {
            Ok(value.to_string())
        }
    }

    fn has_valid_template_name(value: &str) -> Result<String, String> {
        match Self::has_no_commas(value) {
            Ok(no_commas_value) => match Self::has_no_whitespaces(&no_commas_value) {
                Ok(clean_value) => Ok(clean_value),
                Err(whitespaces_error) => Err(whitespaces_error),
            },
            Err(commas_error) => match Self::has_no_whitespaces(value) {
                Ok(_) => Err(commas_error),
                Err(whitespaces_error) => Err(format!("{} + {}", commas_error, whitespaces_error)),
            },
        }
    }

    fn is_starting_with_slash(value: &str) -> Result<String, String> {
        if value.starts_with('/') {
            Ok(value.to_string())
        } else {
            Err(String::from(
                constant::error_messages::URI_WITHOUT_STARTING_SLASH,
            ))
        }
    }

    fn is_valid_url(value: &str) -> Result<String, String> {
        match Url::parse(value) {
            Ok(valid_url) => {
                if valid_url.scheme() == "https" || valid_url.scheme() == "http" {
                    Ok(value.to_string())
                } else {
                    Err(String::from(constant::error_messages::INVALID_URL))
                }
            }
            Err(_) => Err(String::from(constant::error_messages::INVALID_URL)),
        }
    }
}
