use clap::Parser;

use crate::{
    constant,
    validator::{CliArgsValidator, DefaultCliArgsValidator},
};

use std::ffi::OsString;

use crate::ProgramExit;
pub use crate::parser::impls::DefaultArgsParser;

/// Struct to parse and gather cli args parsing result.
///
/// It should not be used directly to parse cli args, but should be
/// used along [`crate::parser::ArgsParser`], which wraps all the complex
/// parsing logic.
#[derive(Parser, Debug, PartialEq, Default)]
#[command(version, author, long_about = None)]
#[command(about = constant::parser_infos::ABOUT)]
#[command(help_template = "\
{before-help}
{usage-heading} {usage}

{about-with-newline}
{all-args}{after-help}

Version: {version}
Author: {author}
")]
#[command(disable_help_flag = true, disable_version_flag = true)]
pub struct Args {
    /// A non-empty list of gitignore template names.
    ///
    /// Represented by the provided positional arguments, and required
    /// unless any of `author`, `version` or `help` options are given.
    #[arg(
        required_unless_present_any = vec!["author", "version", "help"],
        value_parser = DefaultCliArgsValidator::has_no_commas,
        help = constant::help_messages::TEMPLATE_NAMES
    )]
    pub template_names: Vec<String>,

    /// The gitignore template generator service url.
    ///
    /// Optional value represented by the cli option
    /// [`constant::cli_options::SERVER_URL`] that takes a string value, and
    /// falling back to [`constant::template_generator::BASE_URL`] if not
    /// provided in cli args.
    #[arg(
        short = constant::cli_options::SERVER_URL.short,
        long = constant::cli_options::SERVER_URL.long,
        help = constant::help_messages::SERVER_URL,
        default_value = constant::template_generator::BASE_URL
    )]
    pub server_url: String,

    /// The boolean indicator of whether to display help infos or not.
    ///
    /// Optional value represented by the cli option
    /// [`constant::cli_options::HELP`], and falling back to `false` if
    /// not provided in cli args.
    #[arg(
        id = "help",
        short = constant::cli_options::HELP.short,
        long = constant::cli_options::HELP.long,
        action = clap::ArgAction::SetTrue,
        help = constant::help_messages::HELP
    )]
    pub show_help: bool,

    /// The boolean indicator of whether to display version infos or not.
    ///
    /// Optional value represented by the cli option
    /// [`constant::cli_options::VERSION`], and falling back to `false` if
    /// not provided in cli args.
    #[arg(
        id = "version",
        short = constant::cli_options::VERSION.short,
        long = constant::cli_options::VERSION.long,
        action = clap::ArgAction::SetTrue,
        help = constant::help_messages::VERSION
    )]
    pub show_version: bool,

    /// The boolean indicator of whether to display author infos or not.
    ///
    /// Optional value represented by the cli option
    /// [`constant::cli_options::AUTHOR`], and falling back to `false` if
    /// not provided in cli args.
    #[arg(
        id = "author",
        short = constant::cli_options::AUTHOR.short,
        long = constant::cli_options::AUTHOR.long,
        action = clap::ArgAction::SetTrue,
        help = constant::help_messages::AUTHOR
    )]
    pub show_author: bool,
}

impl Args {
    /// Sets new value for `template_names` field.
    ///
    /// It needs to be called on struct instance and effectively mutates it.
    ///
    /// # Arguments
    /// 
    /// * `template_names` - The new value to be assigned to `template_names`
    ///     field.
    ///
    /// # Returns
    ///
    /// The mutated borrowed instance.
    pub fn with_template_names(mut self, template_names: Vec<String>) -> Self {
        self.template_names = template_names;
        self
    }

    /// Sets new value for `server_url` field.
    ///
    /// It needs to be called on struct instance and effectively mutates it.
    ///
    /// # Arguments
    /// 
    /// * `server_url` - The new value to be assigned to `server_url`
    ///     field.
    ///
    /// # Returns
    ///
    /// The mutated borrowed instance.
    pub fn with_server_url(mut self, server_url: &str) -> Self {
        self.server_url = server_url.to_string();
        self
    }
}

pub trait ArgsParser {
    fn parse(args: impl IntoIterator<Item = OsString>) -> Args;
    fn try_parse(
        args: impl IntoIterator<Item = OsString>,
    ) -> Result<Args, ProgramExit>;
}

#[cfg(test)]
mod tests {
    use rstest::*;

    use super::*;
    use crate::helper::*;

    mod default_args_parser {
        use super::*;

        mod try_parse {
            use super::*;

            mod success {
                use crate::{ExitKind, constant};

                use super::*;

                #[rstest]
                #[case("-V")]
                #[case("--version")]
                #[case("-V rust")]
                #[case("rust -V")]
                #[case("rust -s foo -V")]
                #[case("-aV")]
                fn it_parses_version_cli_option(#[case] cli_args: &str) {
                    let cli_args = parse_cli_args(cli_args);
                    let parsed_args = DefaultArgsParser::try_parse(cli_args);

                    let actual_error = parsed_args.as_ref().err();
                    let expected_error = ProgramExit {
                        message: format!(
                            "{} {}",
                            env!("CARGO_PKG_NAME"),
                            env!("CARGO_PKG_VERSION")
                        ),
                        exit_status: 0,
                        styled_message: None,
                        kind: ExitKind::VersionInfos,
                    };
                    let expected_error = Some(&expected_error);

                    assert!(actual_error.is_some());
                    assert_eq!(actual_error, expected_error);
                }

                #[rstest]
                #[case("-h")]
                #[case("--help")]
                #[case("-h rust")]
                #[case("rust -h")]
                #[case("rust -s foo -h")]
                #[case("-aVh")]
                fn it_parses_help_cli_option(#[case] cli_args: &str) {
                    let cli_args = parse_cli_args(cli_args);
                    let parsed_args = DefaultArgsParser::try_parse(cli_args);

                    let actual_error = parsed_args.as_ref().err();
                    let expected_error = ProgramExit {
                        message: get_help_message(),
                        exit_status: 0,
                        styled_message: Some(get_ansi_help_message()),
                        kind: ExitKind::HelpInfos,
                    };
                    let expected_error = Some(&expected_error);

                    assert!(actual_error.is_some());
                    assert_eq!(actual_error, expected_error);
                }

                #[rstest]
                #[case("-a")]
                #[case("--author")]
                #[case("-a rust")]
                #[case("rust -a")]
                #[case("rust -s foo -a")]
                fn it_parses_author_cli_option_preemptively(
                    #[case] cli_args: &str,
                ) {
                    let cli_args = parse_cli_args(cli_args);
                    let parsed_args = DefaultArgsParser::try_parse(cli_args);

                    let actual_error = parsed_args.as_ref().err();
                    let expected_error = ProgramExit {
                        message: env!("CARGO_PKG_AUTHORS").to_string(),
                        exit_status: 0,
                        styled_message: None,
                        kind: ExitKind::AuthorInfos,
                    };
                    let expected_error = Some(&expected_error);

                    assert!(actual_error.is_some());
                    assert_eq!(actual_error, expected_error);
                }

                #[rstest]
                #[case("rust")]
                #[case("rust python node")]
                fn it_parses_pos_args_without_server_url_cli_option(
                    #[case] cli_options: &str,
                ) {
                    let cli_args = parse_cli_args(cli_options);
                    let parsed_args = DefaultArgsParser::try_parse(cli_args);

                    let actual_result = parsed_args.as_ref().ok();
                    let expected_result = Args::default()
                        .with_template_names(make_string_vec(cli_options))
                        .with_server_url(
                            constant::template_generator::BASE_URL,
                        );
                    let expected_result = Some(&expected_result);

                    assert!(actual_result.is_some());
                    assert_eq!(actual_result, expected_result);
                }

                #[rstest]
                #[case("rust -s https://test.com")]
                #[case("rust --server-url https://test.com")]
                fn it_parses_pos_args_with_server_url_cli_option(
                    #[case] cli_args: &str,
                ) {
                    let cli_args = parse_cli_args(cli_args);
                    let parsed_args = DefaultArgsParser::try_parse(cli_args);

                    let actual_result = parsed_args.as_ref().ok();
                    let expected_result = Args::default()
                        .with_template_names(make_string_vec("rust"))
                        .with_server_url("https://test.com");
                    let expected_result = Some(&expected_result);

                    assert!(actual_result.is_some());
                    assert_eq!(actual_result, expected_result);
                }
            }

            mod failure {
                use crate::{ExitKind, constant};

                use super::*;

                #[test]
                fn it_fails_parsing_when_no_pos_args_given() {
                    let cli_args = parse_cli_args("");
                    let parsed_args = DefaultArgsParser::try_parse(cli_args);

                    let actual_error = parsed_args.as_ref().err();
                    let expected_error = ProgramExit {
                        message: load_expectation_file_as_string(
                            "no_pos_args_error",
                        ),
                        exit_status: constant::exit_status::GENERIC,

                        styled_message: Some(load_expectation_file_as_string(
                            "ansi_no_pos_args_error",
                        )),
                        kind: ExitKind::Error,
                    };
                    let expected_error = Some(&expected_error);

                    assert!(actual_error.is_some());
                    assert_eq!(actual_error, expected_error);
                }

                #[test]
                fn it_fails_parsing_when_commas_in_pos_args() {
                    let cli_args = parse_cli_args("python,java");
                    let parsed_args = DefaultArgsParser::try_parse(cli_args);

                    let actual_error = parsed_args.as_ref().err();
                    let expected_error = ProgramExit {
                        message: load_expectation_file_as_string(
                            "comma_pos_args_error",
                        ),
                        exit_status: constant::exit_status::GENERIC,

                        styled_message: Some(load_expectation_file_as_string(
                            "ansi_comma_pos_args_error",
                        )),
                        kind: ExitKind::Error,
                    };
                    let expected_error = Some(&expected_error);

                    assert!(actual_error.is_some());
                    assert_eq!(actual_error, expected_error);
                }

                #[test]
                fn it_fails_parsing_when_server_url_but_no_pos_args() {
                    let cli_args = parse_cli_args("-s https://test.com");
                    let parsed_args = DefaultArgsParser::try_parse(cli_args);

                    let actual_error = parsed_args.as_ref().err();
                    let expected_error = ProgramExit {
                        message: load_expectation_file_as_string(
                            "server_url_no_pos_args_error",
                        ),
                        exit_status: constant::exit_status::GENERIC,

                        styled_message: Some(load_expectation_file_as_string(
                            "ansi_server_url_no_pos_args_error",
                        )),
                        kind: ExitKind::Error,
                    };
                    let expected_error = Some(&expected_error);

                    assert!(actual_error.is_some());
                    assert_eq!(actual_error, expected_error);
                }

                #[test]
                fn it_fails_parsing_when_inexistent_cli_option() {
                    let cli_args = parse_cli_args("-x");
                    let parsed_args = DefaultArgsParser::try_parse(cli_args);

                    let actual_error = parsed_args.as_ref().err();
                    let expected_error = ProgramExit {
                        message: load_expectation_file_as_string(
                            "unexpected_argument_error",
                        ),
                        exit_status: constant::exit_status::GENERIC,
                        styled_message: Some(load_expectation_file_as_string(
                            "ansi_unexpected_argument_error",
                        )),
                        kind: ExitKind::Error,
                    };
                    let expected_error = Some(&expected_error);

                    assert!(actual_error.is_some());
                    assert_eq!(actual_error, expected_error);
                }
            }
        }
    }
}
