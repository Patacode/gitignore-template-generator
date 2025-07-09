//! Constants for error messages to be displayed.

/// An error occurred during [`std::process::Command::output`].
pub const CMD_EXECUTION_FAILURE: &str = "Failed to execute command";

/// An error occurred during HTTP body parsing.
pub const BODY_PARSING_ISSUE: &str = "An error occurred during body parsing";

/// An error occurred while reading a file and converting it to a String
/// instance.
pub const FILE_READ_TO_STRING_FAILURE: &str = "Failed to read expected output file";

/// Commas found in cli positional args.
pub const COMMAS_NOT_ALLOWED: &str = "Commas are not allowed in template names";

/// An error occurred during an api call.
pub const API_CALL_FAILURE: &str = "An error occurred during the API call: {error}";

/// A HTTP error 400 occurred during api call.
pub const HTTP_400: &str = "http status: 400";

/// A HTTP error 404 occurred during api call.
pub const HTTP_404: &str = "http status: 404";

/// User requested author infos but none is available.
pub const AUTHOR_INFOS_NOT_AVAILABLE: &str = "Author information not available.";

/// User requested version infos but none is available.
pub const VERSION_INFOS_NOT_AVAILABLE: &str = "Version information not available.";

/// `White_Space` found in cli positional args.
///
/// `White_Space` is specified in the
/// [Unicode Character Database][ucd] [`PropList.txt`].
///
/// [ucd]: https://www.unicode.org/reports/tr44/
/// [`PropList.txt`]: https://www.unicode.org/Public/UCD/latest/ucd/PropList.txt
pub const WHITESPACES_NOT_ALLOWED: &str = "Whitespace characters are not allowed in template names";

// One or more template names are not supported.
pub const INEXISTENT_TEMPLATE_NAMES: &str = "Following template names are not supported: {templates}.\nFor the list of available template names, try '--list'.";

// Conversion of String to u64.
pub const FAILED_U64_CONVERSION: &str = "Failed to convert to u64";

/// Timeout for current HTTP call.
pub const TIMEOUT: &str = "timeout: global";

/// Invalid utf-8 encoding.
pub const INVALID_ENCODING: &str = "io: stream did not contain valid UTF-8";

/// No slash in front of URI.
pub const URI_WITHOUT_STARTING_SLASH: &str = "URIs must start a slash (/)";

/// Invalid URL.
pub const INVALID_URL: &str = "Value must be a valid URL";

pub const INVALID_SCHEME: &str = "URLs must start with a valid scheme (http or https)";

pub const LOCAL_GENERATION: &str =
    "An error occurred while generating template from local file system";

pub const LOCAL_LISTING: &str = "An error occurred while listing templates from local file system";

pub const UNSUPPORTED_TEMPLATE: &str = "One or more provided template names are not supported\nTo enable robust template names check, retry with '--check'.\nFor the list of available template names, try '--list'.";

pub const READ_HOME_ENV_VAR: &str =
    "An error occurred when trying to read $HOME, which is required for local generation: {error}";

pub const INVALID_MAPPED_URI: &str = "Given URI '{uri}' is not supported in defined map";
