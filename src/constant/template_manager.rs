//! Constants for gitignore template manager service.
use crate::helper::TimeoutUnit;

/// Env variable name pointing to local template directory
pub const HOME_ENV_VAR: &str = "GITIGNORE_TEMPLATE_GENERATOR_HOME";

/// Default local template directory
pub const DEFAULT_HOME: &str = ".gitignore_template_generator";

pub const DEFAULT_TEMPLATE_DIR: &str = "{home_path}/.gitignore_template_generator/templates";

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
