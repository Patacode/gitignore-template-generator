//! Components used to safely generate gitignore
//! templates from a template generator API over HTTP.

pub mod constant;
mod core;
pub mod helper;
pub mod http_client;
pub mod parser;
pub mod validator;

// exposed entities
pub use core::*;
