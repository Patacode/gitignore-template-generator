use std::{ffi::OsString, process::exit};

use clap::Command;

use crate::{ExitKind, ProgramExit, constant};

use super::{Args, ArgsParser, command::build_clap_args};

/// Default implementation of args parser that parses CLI args using
/// [`clap`].
pub struct ClapArgsParser {
    cli_parser: Command,
}

#[allow(clippy::new_without_default)]
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

    fn parse_global_options(&self, args: &Args) -> Option<ProgramExit> {
        if args.show_help {
            let rendered_help = self.cli_parser.clone().render_help();
            Some(ProgramExit {
                message: rendered_help.to_string().trim_end().to_string(),
                exit_status: constant::exit_status::SUCCESS,
                styled_message: Some(
                    rendered_help.ansi().to_string().trim_end().to_string(),
                ),
                kind: ExitKind::HelpInfos,
            })
        } else if args.show_version {
            let message = match self.cli_parser.get_version() {
                Some(version) => {
                    format!("{} {version}", env!("CARGO_PKG_NAME"))
                }
                None => constant::error_messages::VERSION_INFOS_NOT_AVAILABLE
                    .to_string(),
            };

            Some(ProgramExit {
                message,
                exit_status: constant::exit_status::SUCCESS,
                styled_message: None,
                kind: ExitKind::VersionInfos,
            })
        } else if args.show_author {
            let message = String::from(match self.cli_parser.get_author() {
                Some(author) => author,
                None => constant::error_messages::AUTHOR_INFOS_NOT_AVAILABLE,
            });

            Some(ProgramExit {
                message,
                exit_status: constant::exit_status::SUCCESS,
                styled_message: None,
                kind: ExitKind::AuthorInfos,
            })
        } else {
            None
        }
    }

    fn print_error_message(error: &ProgramExit, message: &str) {
        match error.kind {
            ExitKind::VersionInfos
            | ExitKind::HelpInfos
            | ExitKind::AuthorInfos => println!("{message}"),
            ExitKind::Error => eprintln!("{message}"),
        }
    }
}

impl ArgsParser for ClapArgsParser {
    /// Parses given cli args and perform basic error handling.
    ///
    /// * If the underlying [`ProgramExit`] contains a
    ///     [`ProgramExit::styled_message`], it will be printed instead of
    ///     [`ProgramExit::message`].
    /// * Will exit using [`ProgramExit::exit_status`] if any
    ///     [`ProgramExit`] received.
    /// * Will print to stderr on error, to stdout on early exit (i.e. version,
    ///     author, help options)
    ///
    /// See [`ArgsParser::parse`] for more infos.
    fn parse(&self, args: impl IntoIterator<Item = OsString>) -> Args {
        match self.try_parse(args) {
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
        &self,
        args: impl IntoIterator<Item = OsString>,
    ) -> Result<Args, ProgramExit> {
        match self.cli_parser.clone().try_get_matches_from(args) {
            Ok(parsing_result) => {
                let parsed_args = Args {
                    template_names: parsing_result
                        .get_many::<String>("TEMPLATE_NAMES")
                        .map(|vals| vals.cloned().collect())
                        .unwrap_or_default(),

                    server_url: parsing_result
                        .get_one::<String>("SERVER_URL")
                        .unwrap()
                        .to_string(),

                    endpoint_uri: parsing_result
                        .get_one::<String>("ENDPOINT_URI")
                        .unwrap()
                        .to_string(),

                    show_help: parsing_result.get_flag("HELP"),
                    show_version: parsing_result.get_flag("VERSION"),
                    show_author: parsing_result.get_flag("AUTHOR"),
                };

                match self.parse_global_options(&parsed_args) {
                    Some(error) => Err(error),
                    None => Ok(parsed_args),
                }
            }
            Err(error) => Err(ProgramExit {
                message: format!(
                    "{}\nFor more information, try '--help'.",
                    error.render()
                ),
                exit_status: error.exit_code(),
                styled_message: Some(format!(
                    "{}\nFor more information, try '\u{1b}[1m--help\u{1b}[0m'.",
                    error.render().ansi()
                )),
                kind: ExitKind::Error,
            }),
        }
    }
}
