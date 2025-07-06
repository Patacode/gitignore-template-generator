//! Define components to parse cli args.

mod command;
mod impls;

#[cfg(test)]
mod tests;

use std::ffi::OsString;

pub use crate::parser::impls::ClapArgsParser;
use crate::{core::ProgramExit, helper::TimeoutUnit};

pub enum Action {
    List,
    RobustGenerate,
    Generate,
}

/// Struct to gather cli args parsing result.
///
/// Used by [`crate::parser::ArgsParser`] implementations to store
/// parsing result.
#[derive(Debug, PartialEq)]
pub struct Args {
    /// A non-empty list of gitignore template names.
    ///
    /// * Represented by the provided positional arguments, and required
    ///   unless any of `author`, `version` or `help` options are given.
    /// * This field does not allow commas in any of its field.
    pub template_names: Vec<String>,

    /// The gitignore template generator service url.
    ///
    /// * Optional value represented by the cli option
    ///   [`crate::constant::cli_options::SERVER_URL`] that takes a string
    ///   value, and falling back to
    ///   [`crate::constant::template_manager::BASE_URL`] if not provided
    ///   in cli args.
    pub server_url: String,

    /// The gitignore template generator service endpoint uri.
    ///
    /// * Optional value represented by the cli option
    ///   [`crate::constant::cli_options::GENERATOR_URI`] that takes a string
    ///   value, and falling back to
    ///   [`crate::constant::template_manager::GENERATOR_URI`] if not provided in cli
    ///   args.
    pub generator_uri: String,

    /// The gitignore template lister service endpoint uri.
    ///
    /// * Optional value represented by the cli option
    ///   [`crate::constant::cli_options::LISTER_URI`] that takes a string
    ///   value, and falling back to
    ///   [`crate::constant::template_manager::LISTER_URI`] if not provided in cli
    ///   args.
    pub lister_uri: String,

    /// The boolean indicator of whether to display help infos or not.
    ///
    /// * Optional value represented by the cli option
    ///   [`crate::constant::cli_options::HELP`], and falling back to `false`
    ///   if not provided in cli args.
    /// * Has precedence over version and author options if multiple are given
    pub show_help: bool,

    /// The boolean indicator of whether to display version infos or not.
    ///
    /// * Optional value represented by the cli option
    ///   [`crate::constant::cli_options::VERSION`], and falling back to
    ///   `false` if not provided in cli args.
    /// * Has precedence over author option if multiple are given
    pub show_version: bool,

    /// The boolean indicator of whether to display author infos or not.
    ///
    /// * Optional value represented by the cli option
    ///   [`crate::constant::cli_options::AUTHOR`], and falling back to
    ///   `false` if not provided in cli args.
    pub show_author: bool,

    /// The boolean indicator of whether to display list of available templates
    /// or not.
    ///
    /// * Optional value represented by the cli option
    ///   [`crate::constant::cli_options::LIST`], and falling back to
    ///   `false` if not provided in cli args.
    pub show_list: bool,

    /// The boolean indicator of whether to enable robust template check or not.
    ///
    /// Robust template check allow the script to handle template existence
    /// check without reaching the generator endpoint.
    ///
    /// * Optional value represented by the cli option
    ///   [`crate::constant::cli_options::CHECK`], and falling back to
    ///   `false` if not provided in cli args.
    pub check_template_names: bool,

    /// The service call timeout.
    ///
    /// * Optional value represented by the cli option
    ///   [`crate::constant::cli_options::TIMEOUT`], and falling back to
    ///   [`crate::constant::template_manager::TIMEOUT`] if not provided in
    ///   cli args.
    pub timeout: u64,

    /// The timeout unit.
    ///
    /// * Optional value represented by the cli option
    ///   [`crate::constant::cli_options::TIMEOUT_UNIT`], and falling back to
    ///   [`crate::constant::template_manager::TIMEOUT_UNIT`] if not provided in
    ///   cli args.
    pub timeout_unit: TimeoutUnit,
}

/// Cli args parser trait to parse CLI args and return them in an [`Args`].
///
/// The produced Args instance needs to comply with constraints of each
/// one of its fields (see fields doc in [`Args`] for more infos).
pub trait ArgsParser {
    /// Parses given cli args and return them as an [`Args`] instance.
    ///
    /// * First CLI args should be the binary name
    /// * Rely on [`ArgsParser::try_parse`] method but additionally wrap
    ///   error handling logic
    ///
    /// # Arguments
    ///
    /// * `args` - The CLI args to be parsed. Typically retrieved from
    ///   [`std::env::args_os`].
    ///
    /// # Returns
    ///
    /// An owned instance of [`Args`] containing parsing result of given args.
    fn parse(&self, args: impl IntoIterator<Item = OsString>) -> Args;

    /// Parses given cli args and return them as an [`Args`] instance if no
    /// error or early exit occurred.
    ///
    /// * First CLI args should be the binary name
    /// * Version, author and help options are considered as early program
    ///   exit
    /// * Returned Args complies with expected constraints (see fields doc
    ///   in [`Args`] for more infos)
    ///
    /// # Arguments
    ///
    /// * `args` - The CLI args to be parsed. Typically retrieved from
    ///   [`std::env::args_os`].
    ///
    /// # Returns
    ///
    /// A result containing an owned instance of [`Args`] if successful parsing,
    /// or a [`ProgramExit`] if any error or early exit occurred (e.g. version/
    /// author/help infos printing, invalid cli args...)
    fn try_parse(&self, args: impl IntoIterator<Item = OsString>) -> Result<Args, ProgramExit>;
}
