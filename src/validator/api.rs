pub use crate::validator::impls::DefaultCliArgsValidator;

/// Cli args validator trait to validate cli args.
pub trait CliArgsValidator {
    /// Checks if given value contains commas or not.
    ///
    /// # Arguments
    ///
    /// `value` - The value to be checked
    ///
    /// # Returns
    ///
    /// A result containing the provided value if no commas found, or
    /// an error containing proper error message.
    fn has_no_commas(value: &str) -> Result<String, String>;

    /// Checks if given value contains `White_Space` or not.
    ///
    /// `White_Space` is specified in the
    /// [Unicode Character Database][ucd] [`PropList.txt`].
    ///
    /// [ucd]: https://www.unicode.org/reports/tr44
    /// [`PropList.txt`]: https://www.unicode.org/Public/UCD/latest/ucd/PropList.txt
    ///
    /// # Arguments
    ///
    /// `value` - The value to be checked
    ///
    /// # Returns
    ///
    /// A result containing the provided value if no `White_Space` found, or
    /// an error containing proper error message.
    fn has_no_whitespaces(value: &str) -> Result<String, String>;

    /// Checks if given value is a valid template name.
    ///
    /// A valid template name does not contains commas or whitespaces, as
    /// defined by [`CliArgsValidator::has_no_commas`] and
    /// [`CliArgsValidator::has_no_whitespaces`] respectively.
    ///
    /// # Arguments
    ///
    /// `value` - The value to be checked
    ///
    /// # Returns
    ///
    /// A result containing the provided value if valid, or
    /// an error containing proper error message.
    fn has_valid_template_name(value: &str) -> Result<String, String>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::constant;

    mod default_cli_args_validator {
        use super::*;

        mod has_no_commas {
            use super::*;

            mod success {
                use super::*;

                #[test]
                fn it_returns_ok_for_comma_free_values() {
                    let value = "rust";

                    let expected: Result<String, String> =
                        Ok(String::from(value));
                    let actual = DefaultCliArgsValidator::has_no_commas(value);

                    assert_eq!(actual, expected);
                }
            }

            mod failure {
                use super::*;

                #[test]
                fn it_returns_error_for_values_with_commas() {
                    let value = "r,ust";

                    let expected: Result<String, String> = Err(String::from(
                        constant::error_messages::COMMAS_NOT_ALLOWED,
                    ));
                    let actual = DefaultCliArgsValidator::has_no_commas(value);

                    assert_eq!(actual, expected);
                }
            }
        }

        mod has_no_whitespaces {
            use super::*;

            mod success {
                use super::*;

                #[test]
                fn it_returns_ok_for_whitespace_free_values() {
                    let value = "rust";

                    let expected: Result<String, String> =
                        Ok(String::from(value));
                    let actual =
                        DefaultCliArgsValidator::has_no_whitespaces(value);

                    assert_eq!(actual, expected);
                }
            }

            mod failure {
                use super::*;

                #[test]
                fn it_returns_error_for_values_with_whitespaces() {
                    let value = "r ust";

                    let expected: Result<String, String> = Err(String::from(
                        constant::error_messages::WHITESPACES_NOT_ALLOWED,
                    ));
                    let actual =
                        DefaultCliArgsValidator::has_no_whitespaces(value);

                    assert_eq!(actual, expected);
                }
            }
        }

        mod has_valid_template_name {
            use super::*;

            mod success {
                use super::*;

                #[test]
                fn it_returns_ok_for_comma_and_whitespace_free_values() {
                    let value = "rust";

                    let expected: Result<String, String> =
                        Ok(String::from(value));
                    let actual =
                        DefaultCliArgsValidator::has_valid_template_name(value);

                    assert_eq!(actual, expected);
                }
            }

            mod failure {
                use super::*;

                #[test]
                fn it_returns_error_for_values_with_whitespaces() {
                    let value = "r ust";

                    let expected: Result<String, String> = Err(String::from(
                        constant::error_messages::WHITESPACES_NOT_ALLOWED,
                    ));
                    let actual =
                        DefaultCliArgsValidator::has_valid_template_name(value);

                    assert_eq!(actual, expected);
                }

                #[test]
                fn it_returns_error_for_values_with_commas() {
                    let value = "r,ust";

                    let expected: Result<String, String> = Err(String::from(
                        constant::error_messages::COMMAS_NOT_ALLOWED,
                    ));
                    let actual =
                        DefaultCliArgsValidator::has_valid_template_name(value);

                    assert_eq!(actual, expected);
                }

                #[test]
                fn it_returns_error_for_values_with_commas_and_whitespaces() {
                    let value = "r, ust";

                    let expected: Result<String, String> = Err(format!(
                        "{} + {}",
                        constant::error_messages::COMMAS_NOT_ALLOWED,
                        constant::error_messages::WHITESPACES_NOT_ALLOWED,
                    ));
                    let actual =
                        DefaultCliArgsValidator::has_valid_template_name(value);

                    assert_eq!(actual, expected);
                }
            }
        }
    }
}
