//! Define components to validate cli args.
pub use crate::validator::impls::DefaultCliArgsValidator;

mod impls;

#[cfg(test)]
mod tests;

/// Cli args validator trait to validate cli args.
pub trait CliArgsValidator {
    /// Checks if given value does not contain any commas.
    ///
    /// # Arguments
    ///
    /// `value` - The value to be checked
    ///
    /// # Returns
    ///
    /// A result containing the provided value if no commas found, or
    /// an error containing proper error message otherwise.
    fn has_no_commas(value: &str) -> Result<String, String>;

    /// Checks if given value does not contain any `White_Space` characters.
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
    /// an error containing proper error message otherwise.
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
    /// A result containing the provided value if valid template name, or
    /// an error containing proper error message otherwise.
    fn is_valid_template_name(value: &str) -> Result<String, String>;

    /// Checks if given value starts with a slash (`/`).
    ///
    /// # Arguments
    ///
    /// `value` - The value to be checked
    ///
    /// # Returns
    ///
    /// A result containing the provided value if a slash was found as first
    /// character, or an error containing proper error message otherwise.
    fn is_starting_with_slash(value: &str) -> Result<String, String>;

    /// Checks if given value is a valid URL.
    ///
    /// URL validity is checked against the [URL Standard].
    ///
    /// [URL Standard]: https://url.spec.whatwg.org/
    ///
    /// # Arguments
    ///
    /// `value` - The value to be checked
    ///
    /// # Returns
    ///
    /// A result containing the provided value if valid url, or an error
    /// containing proper error message otherwise.
    fn is_valid_url(value: &str) -> Result<String, String>;
}
