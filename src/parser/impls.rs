use std::{ffi::OsString, process::exit};

use clap::{CommandFactory, Parser};

use crate::{ErrorKind, ProgramError, config::Args, constant};

use super::ArgsParser;

pub struct DefaultArgsParser;

impl DefaultArgsParser {
    fn parse_global_options(args: &Args) -> Option<ProgramError> {
        if args.show_help {
            let mut cmd = Args::command();
            let rendered_help = cmd.render_help();
            Some(ProgramError {
                message: rendered_help.to_string().trim_end().to_string(),
                exit_status: constant::exit_status::SUCCESS,
                styled_message: Some(
                    rendered_help.ansi().to_string().trim_end().to_string(),
                ),
                error_kind: ErrorKind::HelpInfos,
            })
        } else if args.show_version {
            let cmd = Args::command();
            let message = match cmd.get_version() {
                Some(version) => {
                    format!("{} {version}", env!("CARGO_PKG_NAME"))
                }
                None => constant::error_messages::VERSION_INFOS_NOT_AVAILABLE
                    .to_string(),
            };

            Some(ProgramError {
                message,
                exit_status: constant::exit_status::SUCCESS,
                styled_message: None,
                error_kind: ErrorKind::VersionInfos,
            })
        } else if args.show_author {
            let cmd = Args::command();
            let message = String::from(match cmd.get_author() {
                Some(author) => author,
                None => constant::error_messages::AUTHOR_INFOS_NOT_AVAILABLE,
            });

            Some(ProgramError {
                message,
                exit_status: constant::exit_status::SUCCESS,
                styled_message: None,
                error_kind: ErrorKind::AuthorInfos,
            })
        } else {
            None
        }
    }

    fn print_error_message(error: &ProgramError, message: &str) {
        match error.error_kind {
            ErrorKind::VersionInfos
            | ErrorKind::HelpInfos
            | ErrorKind::AuthorInfos => println!("{message}"),
            ErrorKind::Other => eprintln!("{message}"),
        }
    }
}

impl ArgsParser for DefaultArgsParser {
    fn parse(args: impl IntoIterator<Item = OsString>) -> Args {
        match DefaultArgsParser::try_parse(args) {
            Ok(parsed_args) => parsed_args,
            Err(error) => {
                if let Some(value) = &error.styled_message {
                    Self::print_error_message(&error, value);
                } else {
                    Self::print_error_message(&error, &error.message);
                }

                exit(error.exit_status);
            }
        }
    }

    fn try_parse(
        args: impl IntoIterator<Item = OsString>,
    ) -> Result<Args, ProgramError> {
        match Args::try_parse_from(args) {
            Ok(parsed_args) => match Self::parse_global_options(&parsed_args) {
                Some(error) => Err(error),
                None => Ok(parsed_args),
            },
            Err(error) => Err(ProgramError {
                message: format!(
                    "{}\nFor more information, try '--help'.",
                    error.render()
                ),
                exit_status: error.exit_code(),
                styled_message: Some(format!(
                    "{}\nFor more information, try '\u{1b}[1m--help\u{1b}[0m'.",
                    error.render().ansi()
                )),
                error_kind: ErrorKind::Other,
            }),
        }
    }
}
