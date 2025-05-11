//! Define components used to wrap core logic.
//!
//! As per crate definition, core logic is defined as generating
//! gitignore templates via a template generator API over HTTP. So you will
//! find methods to respond to that need.

mod api;
mod impls;

#[cfg(test)]
mod tests;

pub use api::*;
