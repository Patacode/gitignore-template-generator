//! Modules used to define globally-shared constants.

pub mod error_messages {
    //! Constants for error messages to be displayed.

    /// An error occurred during [`std::process::Command::output`].
    pub const CMD_EXECUTION_FAILURE: &str = "Failed to execute command";

    /// An error occurred during HTTP body parsing.
    pub const BODY_PARSING_ISSUE: &str =
        "An error occurred during body parsing";

    /// An error occurred while reading a file and converting it to a String
    /// instance.
    pub const FILE_READ_TO_STRING_FAILURE: &str =
        "Failed to read expected output file";

    /// Actual result does not match expected regex.
    pub const REGEX_NO_MATCH: &str =
        "Actual output <{actual}> did not match expected pattern <{expected}>";

    /// Commas found in cli positional args.
    pub const COMMAS_NOT_ALLOWED: &str =
        "Commas are not allowed in template names";

    /// An error occurred during an api call.
    pub const API_CALL_FAILURE: &str =
        "An error occurred during the API call: {error}";

    /// A HTTP error 400 occurred during api call.
    pub const HTTP_400: &str = "http status: 400";

    /// User requested author infos but none is available.
    pub const AUTHOR_INFOS_NOT_AVAILABLE: &str =
        "Author information not available.";

    /// User requested version infos but none is available.
    pub const VERSION_INFOS_NOT_AVAILABLE: &str =
        "Version information not available.";
}

pub mod help_messages {
    //! Constants for help messages to be displayed.

    /// Help message bound to [`crate::config::Args::template_names`]
    /// field (i.e. positional args). 
    pub const TEMPLATE_NAMES: &str =
        "A non-empty list of gitignore template names";

    /// Help message bound to [`crate::config::Args::show_author`]
    /// field (i.e. author option).
    pub const AUTHOR: &str = "Print author";

    /// Help message bound to [`crate::config::Args::server_url`]
    /// field (i.e. server url option).
    pub const SERVER_URL: &str = "The gitignore template generator service url";

    /// Help message bound to [`crate::config::Args::show_help`]
    /// field (i.e. help option).
    pub const HELP: &str = "Print help";

    /// Help message bound to [`crate::config::Args::show_version`]
    /// field (i.e. version option).
    pub const VERSION: &str = "Print version";
}

pub mod cli_options {
    //! Constants for short and long cli options specifier.

    use crate::helper::CliOptionName;

    /// Short and long specifier for author option.
    /// 
    /// **Value**: `-a --author`
    pub const AUTHOR: CliOptionName = CliOptionName {
        short: 'a',
        long: "author",
    };

    /// Short and long specifier for server url option.
    /// 
    /// **Value**: `-s --server-url`
    pub const SERVER_URL: CliOptionName = CliOptionName {
        short: 's',
        long: "server-url",
    };

    /// Short and long specifier for help option.
    /// 
    /// **Value**: `-h --help`
    pub const HELP: CliOptionName = CliOptionName {
        short: 'h',
        long: "help",
    };

    /// Short and long specifier for version option.
    /// 
    /// **Value**: `-V --version`
    pub const VERSION: CliOptionName = CliOptionName {
        short: 'V',
        long: "version",
    };
}

pub mod parser_infos {
    //! Constants for general parser infos. 

    /// About text to be displayed when requesting help.
    pub const ABOUT: &str = "Generate templates for .gitignore files";
}

pub mod exit_status {
    //! Constants for script exit status code.

    /// Exit status code for successful script execution.
    pub const SUCCESS: i32 = 0;

    /// Exit status code for generic script error.
    pub const GENERIC: i32 = 2;

    /// Exit status code for HTTP body parsing error.
    pub const BODY_PARSING_ISSUE: i32 = 3;
}

pub mod template_generator {
    //! Constants for template generator service.

    /// Template generator service base URL.
    pub const BASE_URL: &str = "https://www.toptal.com";

    /// Template generator service URI.
    /// 
    /// Used in conjunction with [`BASE_URL`] to build full URL and make
    /// API call.
    pub const URI: &str = "/developers/gitignore/api";
}

pub mod path {
    //! Constants for file or directory path

    /// Path to directory containing test output expectations.
    pub const TEST_EXPECTATIONS: &str = "tests/expected";
}

pub mod regex {
    //! Constants for regexes.

    /// Regex for version specifier.
    pub const SEMVER_VERSION: &str = r"[0-9]+\.[0-9]+\.[0-9]+";
}
