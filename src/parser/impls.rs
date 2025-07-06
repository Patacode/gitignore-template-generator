use std::{ffi::OsString, process::exit};

use clap::Command;

use super::{Args, ArgsParser, command::build_clap_args};
use crate::{
    constant,
    core::{ExitKind, ProgramExit},
};

/// Default implementation of args parser that parses CLI args using
/// [`clap`].
pub struct ClapArgsParser {
    cli_parser: Command,
}

impl ClapArgsParser {
    pub fn new() -> Self {
        Self {
            cli_parser: Command::new(env!("CARGO_PKG_NAME"))
                .version(env!("CARGO_PKG_VERSION"))
                .author(env!("CARGO_PKG_AUTHORS"))
                .about(constant::parser_infos::ABOUT)
                .help_template(include_str!("../../assets/help_template.txt"))
                .disable_help_flag(true)
                .disable_version_flag(true)
                .args(build_clap_args()),
        }
    }

    fn print_message(error: &ProgramExit, message: &str) -> Option<String> {
        match error.kind {
            ExitKind::Error => eprintln!("{message}"),
            _ => println!("{message}"),
        }
        Some(message.to_string())
    }
}

impl ArgsParser for ClapArgsParser {
    /// Parses given cli args and perform basic error handling.
    ///
    /// * If the underlying [`ProgramExit`] contains a
    ///   [`ProgramExit::styled_message`], it will be printed instead of
    ///   [`ProgramExit::message`].
    /// * Will exit using [`ProgramExit::exit_status`] if any
    ///   [`ProgramExit`] received.
    /// * Will print to stderr on error, to stdout on early exit (i.e. version,
    ///   author, help options)
    ///
    /// See [`ArgsParser::parse`] for more infos.
    fn parse(&self, args: impl IntoIterator<Item = OsString>) -> Args {
        match self.try_parse(args) {
            Ok(parsed_args) => parsed_args,
            Err(error) => {
                error
                    .styled_message
                    .clone()
                    .and_then(|styled_message| Self::print_message(&error, &styled_message))
                    .or_else(|| Self::print_message(&error, &error.message));
                exit(error.exit_status);
            }
        }
    }

    fn try_parse(&self, args: impl IntoIterator<Item = OsString>) -> Result<Args, ProgramExit> {
        match self.cli_parser.clone().try_get_matches_from(args) {
            Ok(arg_matches) => {
                let args = Args::from_arg_matches(&arg_matches);
                match args.as_program_exit(&self.cli_parser) {
                    Some(value) => Err(value),
                    None => Ok(args),
                }
            }
            Err(error) => Err(ProgramExit::from_clap_error(&error)),
        }
    }
}
