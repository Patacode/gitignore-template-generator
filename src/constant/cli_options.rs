//! Constants for short and long cli options specifier.
use crate::helper::CliOptionName;

/// Short and long specifier for author option.
///
/// **Value**: `-a --author`
pub const AUTHOR: CliOptionName = CliOptionName {
    short: "a",
    long: "author",
};

/// Short and long specifier for server url option.
///
/// **Value**: `-s --server-url`
pub const SERVER_URL: CliOptionName = CliOptionName {
    short: "s",
    long: "server-url",
};

/// Short and long specifier for help option.
///
/// **Value**: `-h --help`
pub const HELP: CliOptionName = CliOptionName {
    short: "h",
    long: "help",
};

/// Short and long specifier for version option.
///
/// **Value**: `-V --version`
pub const VERSION: CliOptionName = CliOptionName {
    short: "V",
    long: "version",
};

/// Short and long specifier for generator uri option.
///
/// **Value**: `-g --generator-uri`
pub const GENERATOR_URI: CliOptionName = CliOptionName {
    short: "g",
    long: "generator-uri",
};

/// Short and long specifier for template list option.
///
/// **Value**: `-l --list`
pub const LIST: CliOptionName = CliOptionName {
    short: "l",
    long: "list",
};

/// Short and long specifier for lister uri option.
///
/// **Value**: `-i --lister-uri`
pub const LISTER_URI: CliOptionName = CliOptionName {
    short: "i",
    long: "lister-uri",
};

/// Short and long specifier for check option.
///
/// **Value**: `-c --check`
pub const CHECK: CliOptionName = CliOptionName {
    short: "c",
    long: "check",
};

/// Short and long specifier for timeout option.
///
/// **Value**: `-t --timeout`
pub const TIMEOUT: CliOptionName = CliOptionName {
    short: "t",
    long: "timeout",
};

/// Short and long specifier for timeout option.
///
/// **Value**: `-u --timeout-unit`
pub const TIMEOUT_UNIT: CliOptionName = CliOptionName {
    short: "u",
    long: "timeout-unit",
};
