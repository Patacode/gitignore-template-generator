pub mod error_messages {
    pub const CMD_EXECUTION_FAILURE: &str = "Failed to execute command";
    pub const BODY_PARSING_ISSUE: &str =
        "An error occurred during body parsing";
}

pub mod exit_status {
    pub const GENERIC: i32 = 2;
    pub const BODY_PARSING_ISSUE: i32 = 3;
}

pub mod template_generator {
    pub const URI: &str = "/developers/gitignore/api";
}
