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

    pub const INEXISTENT_TEMPLATE_NAMES: &str = "Following template names are not supported: {templates}. For the list of available template names, try '--list'.";
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
    /// field (i.e. template list option).
    pub const LIST: &str = "List available templates";

    /// Help message bound to [`crate::parser::Args::lister_uri`]
    /// field (i.e. template list option).
    pub const LISTER_URI: &str = "The template lister uri";
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

pub mod template_manager {
    //! Constants for gitignore template manager service.

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
}

pub mod path {
    //! Constants for file or directory path.

    /// Path to directory containing test output expectations.
    pub const TEST_EXPECTATIONS: &str = "tests/expected";
}
