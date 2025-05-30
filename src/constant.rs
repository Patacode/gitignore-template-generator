//! Define globally-shared constants.

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

    /// Commas found in cli positional args.
    pub const COMMAS_NOT_ALLOWED: &str =
        "Commas are not allowed in template names";

    /// An error occurred during an api call.
    pub const API_CALL_FAILURE: &str =
        "An error occurred during the API call: {error}";

    /// A HTTP error 400 occurred during api call.
    pub const HTTP_400: &str = "http status: 400";

    /// A HTTP error 404 occurred during api call.
    pub const HTTP_404: &str = "http status: 404";

    /// User requested author infos but none is available.
    pub const AUTHOR_INFOS_NOT_AVAILABLE: &str =
        "Author information not available.";

    /// User requested version infos but none is available.
    pub const VERSION_INFOS_NOT_AVAILABLE: &str =
        "Version information not available.";

    /// `White_Space` found in cli positional args.
    ///
    /// `White_Space` is specified in the
    /// [Unicode Character Database][ucd] [`PropList.txt`].
    ///
    /// [ucd]: https://www.unicode.org/reports/tr44/
    /// [`PropList.txt`]: https://www.unicode.org/Public/UCD/latest/ucd/PropList.txt
    pub const WHITESPACES_NOT_ALLOWED: &str =
        "Whitespace characters are not allowed in template names";

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

    pub const LOCAL_GENERATION: &str =
        "An error occurred while generating template from local file system";

    pub const LOCAL_LISTING: &str =
        "An error occurred while listing templates from local file system";

    pub const UNSUPPORTED_TEMPLATE: &str = "One or more provided template names are not supported\nTo enable robust template names check, retry with '--check'.\nFor the list of available template names, try '--list'.";
}

pub mod help_messages {
    //! Constants for help messages to be displayed.

    /// Help message bound to [`crate::parser::Args::template_names`]
    /// field (i.e. positional args).
    pub const TEMPLATE_NAMES: &str =
        "A non-empty list of gitignore template names";

    /// Help message bound to [`crate::parser::Args::show_author`]
    /// field (i.e. author option).
    pub const AUTHOR: &str = "Print author";

    /// Help message bound to [`crate::parser::Args::server_url`]
    /// field (i.e. server url option).
    pub const SERVER_URL: &str = "The template manager url";

    /// Help message bound to [`crate::parser::Args::show_help`]
    /// field (i.e. help option).
    pub const HELP: &str = "Print help";

    /// Help message bound to [`crate::parser::Args::show_version`]
    /// field (i.e. version option).
    pub const VERSION: &str = "Print version";

    /// Help message bound to [`crate::parser::Args::generator_uri`]
    /// field (i.e. generator uri option).
    pub const GENERATOR_URI: &str = "The template generator uri";

    /// Help message bound to [`crate::parser::Args::show_list`]
    /// field (i.e. list option).
    pub const LIST: &str = "List available templates";

    /// Help message bound to [`crate::parser::Args::lister_uri`]
    /// field (i.e. lister uri option).
    pub const LISTER_URI: &str = "The template lister uri";

    /// Help message bound to [`crate::parser::Args::check_template_names`]
    /// field (i.e. check option).
    pub const CHECK: &str = "Enable robust template names check";

    /// Help message bound to [`crate::parser::Args::timeout`]
    /// field (i.e. timeout option).
    pub const TIMEOUT: &str =
        "The template generation and listing service calls timeout";

    /// Help message bound to [`crate::parser::Args::timeout_unit`]
    /// field (i.e. timeout unit option).
    pub const TIMEOUT_UNIT: &str = "The timeout unit";
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

    /// Short and long specifier for generator uri option.
    ///
    /// **Value**: `-g --generator-uri`
    pub const GENERATOR_URI: CliOptionName = CliOptionName {
        short: 'g',
        long: "generator-uri",
    };

    /// Short and long specifier for template list option.
    ///
    /// **Value**: `-l --list`
    pub const LIST: CliOptionName = CliOptionName {
        short: 'l',
        long: "list",
    };

    /// Short and long specifier for lister uri option.
    ///
    /// **Value**: `-i --lister-uri`
    pub const LISTER_URI: CliOptionName = CliOptionName {
        short: 'i',
        long: "lister-uri",
    };

    /// Short and long specifier for check option.
    ///
    /// **Value**: `-c --check`
    pub const CHECK: CliOptionName = CliOptionName {
        short: 'c',
        long: "check",
    };

    /// Short and long specifier for timeout option.
    ///
    /// **Value**: `-t --timeout`
    pub const TIMEOUT: CliOptionName = CliOptionName {
        short: 't',
        long: "timeout",
    };

    /// Short and long specifier for timeout option.
    ///
    /// **Value**: `-u --timeout-unit`
    pub const TIMEOUT_UNIT: CliOptionName = CliOptionName {
        short: 'u',
        long: "timeout-unit",
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

    /// Exit status code for any error from HTTP client itself.
    pub const HTTP_CLIENT_ERROR: i32 = 4;
}

pub mod template_manager {
    //! Constants for gitignore template manager service.

    use crate::parser::TimeoutUnit;

    /// Env variable name pointing to local template directory
    pub const HOME_ENV_VAR: &str = "GITIGNORE_TEMPLATE_GENERATOR_HOME";

    /// Default local template directory
    pub const DEFAULT_HOME: &str = ".gitignore_template_generator";

    /// Template manager service base URL.
    pub const BASE_URL: &str = "https://www.toptal.com";

    /// Template generator service URI.
    ///
    /// Used in conjunction with [`BASE_URL`] to build full URL and make
    /// API call.
    pub const GENERATOR_URI: &str = "/developers/gitignore/api";

    /// Template lister service URI.
    ///
    /// Used in conjunction with [`BASE_URL`] to build full URL and make
    /// API call.
    pub const LISTER_URI: &str = "/developers/gitignore/api/list";

    /// Timeout in seconds for HTTP calls to generator/lister service (str version).
    pub const TIMEOUT: &str = "5";

    /// Timeout in milliseconds for HTTP calls to generator/lister service (str version).
    pub const TIMEOUT_MILLISECOND: &str = "5000";

    /// Timeout in seconds for HTTP calls to generator/lister service (integer version).
    pub const TIMEOUT_INT: u64 = 5;

    /// Timeout milliseconds for HTTP calls to generator/lister service (integer version).
    pub const TIMEOUT_MILLISECOND_INT: u64 = 5000;

    /// Second timeout unit (str version).
    pub const TIMEOUT_UNIT: &str = "second";

    /// Millisecond timeout unit (str version).
    pub const TIMEOUT_MILLISECOND_UNIT: &str = "millisecond";

    /// Second timeout unit (enum version).
    ///
    /// `value` - TimeoutUnit::SECOND
    pub const TIMEOUT_UNIT_ENUM: TimeoutUnit = TimeoutUnit::SECOND;
}

pub mod path {
    //! Constants for file or directory path.

    /// Path to directory containing test output expectations.
    pub const TEST_EXPECTATIONS: &str = "tests/expected";

    /// Path to directory containing test resources.
    pub const TEST_RESOURCES: &str = "tests/resources";
}
