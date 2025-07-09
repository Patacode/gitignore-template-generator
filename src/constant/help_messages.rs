//! Constants for help messages to be displayed.

/// Help message bound to [`crate::parser::Args::template_names`]
/// field (i.e. positional args).
pub const TEMPLATE_NAMES: &str = "A non-empty list of gitignore template names";

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
pub const TIMEOUT: &str = "The template generation and listing service calls timeout";

/// Help message bound to [`crate::parser::Args::timeout_unit`]
/// field (i.e. timeout unit option).
pub const TIMEOUT_UNIT: &str = "The timeout unit";
