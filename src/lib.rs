//! Components used to safely generate gitignore
//! templates from a template generator API over HTTP.

pub mod constant; // global constants
mod core; // core api
pub mod helper; // internal helper logic
pub mod http_client; // http client to make api calls
pub mod parser; // cli args parser
pub mod validator; // cli args validators

// exposed entities
pub use core::*;
