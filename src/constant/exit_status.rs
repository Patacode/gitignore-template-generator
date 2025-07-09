//! Constants for script exit status code.

/// Exit status code for successful script execution.
pub const SUCCESS: i32 = 0;

/// Exit status code for generic script error.
pub const GENERIC: i32 = 2;

/// Exit status code for HTTP body parsing error.
pub const BODY_PARSING_ISSUE: i32 = 3;

/// Exit status code for any error from HTTP client itself.
pub const HTTP_CLIENT_ERROR: i32 = 4;
