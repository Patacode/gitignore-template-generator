pub const NOTHING_TO_BE_PRINTED: &str = "Nothing to be printed";

pub const ENV_VAR_RESET: &str = "{name} env var was set to {value}. Resetting it...";
pub const ENV_VAR_REMOVAL_BEFORE: &str = "{name} is set. Removing it...";
pub const ENV_VAR_REMOVAL_AFTER: &str = "{name} env var wasn't set. Removing it...";

pub const RESET: &str = "Reset!";
pub const REMOVED: &str = "Removed!";

pub const TEST_CXT_DROPPED: &str = "Test context dropped!";
pub const TEST_CTX_CREATED: &str = "Test context created!";

pub const DEFAULT_TIMEOUT: &str = "{second}s/{millis}ms";

pub const HELP_FOR_MORE_INFOS: &str = "{error}\nFor more information, try '--help'.";
pub const STYLED_HELP_FOR_MORE_INFOS: &str =
    "{error}\nFor more information, try '\u{1b}[1m--help\u{1b}[0m'.";
