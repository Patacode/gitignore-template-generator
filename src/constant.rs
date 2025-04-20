pub mod error_messages {
    pub const CMD_EXECUTION_FAILURE: &str = "Failed to execute command";
    pub const BODY_PARSING_ISSUE: &str =
        "An error occurred during body parsing";
    pub const FILE_READ_TO_STRING_FAILURE: &str =
        "Failed to read expected output file";
    pub const REGEX_NO_MATCH: &str =
        "Actual output <{actual}> did not match expected pattern <{expected}>";
    pub const COMMAS_NOT_ALLOWED: &str =
        "Commas are not allowed in template names";
    pub const API_CALL_FAILURE: &str =
        "An error occurred during the API call: {error}";
    pub const HTTP_400: &str = "http status: 400";
    pub const AUTHOR_INFOS_NOT_AVAILABLE: &str =
        "Author information not available.";
}

pub mod help_messages {
    pub const TEMPLATE_NAMES: &str =
        "A non-empty list of existing gitignore template names";
    pub const AUTHOR: &str = "Print author";
    pub const SERVER_URL: &str =
        "The url of the server hosting gitignore template generator service";
    pub const HELP: &str = "Print help";
    pub const VERSION: &str = "Print version";
}

pub mod cli_options {
    use crate::helper::CliOptionName;

    pub const AUTHOR: CliOptionName = CliOptionName {
        short: 'a',
        long: "author",
    };
    pub const SERVER_URL: CliOptionName = CliOptionName {
        short: 's',
        long: "server-url",
    };
    pub const HELP: CliOptionName = CliOptionName {
        short: 'h',
        long: "help",
    };
    pub const VERSION: CliOptionName = CliOptionName {
        short: 'V',
        long: "version",
    };
}

pub mod parser_infos {
    pub const ABOUT: &str = "Generate templates for .gitignore files";
}

pub mod exit_status {
    pub const SUCCESS: i32 = 0;
    pub const GENERIC: i32 = 2;
    pub const BODY_PARSING_ISSUE: i32 = 3;
}

pub mod template_generator {
    pub const BASE_URL: &str = "https://www.toptal.com";
    pub const URI: &str = "/developers/gitignore/api";
}

pub mod path {
    pub const TEST_EXPECTATIONS: &str = "tests/expected";
}

pub mod regex {
    pub const SEMVER_VERSION: &str = r"[0-9]+\.[0-9]+\.[0-9]+";
}
