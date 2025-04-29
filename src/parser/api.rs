use std::ffi::OsString;

use crate::ProgramExit;
pub use crate::parser::impls::ClapArgsParser;

/// Struct to gather cli args parsing result.
///
/// Used by [`crate::parser::ArgsParser`] implementations to store
/// parsing result.
#[derive(Debug, PartialEq, Default)]
pub struct Args {
    /// A non-empty list of gitignore template names.
    ///
    /// * Represented by the provided positional arguments, and required
    ///     unless any of `author`, `version` or `help` options are given.
    /// * This field does not allow commas in any of its field.
    pub template_names: Vec<String>,

    /// The gitignore template generator service url.
    ///
    /// * Optional value represented by the cli option
    ///     [`crate::constant::cli_options::SERVER_URL`] that takes a string
    ///     value, and falling back to
    ///     [`crate::constant::template_generator::BASE_URL`] if not provided
    ///     in cli args.
    pub server_url: String,

    /// The gitignore template generator service endpoint uri.
    ///
    /// * Optional value represented by the cli option
    ///     [`crate::constant::cli_options::ENDPOINT_URI`] that takes a string
    ///     value, and falling back to
    ///     [`crate::constant::template_generator::URI`] if not provided in cli
    ///     args.
    pub endpoint_uri: String,

    /// The boolean indicator of whether to display help infos or not.
    ///
    /// * Optional value represented by the cli option
    ///     [`crate::constant::cli_options::HELP`], and falling back to `false`
    ///     if not provided in cli args.
    /// * Has precedence over version and author options if multiple are given
    pub show_help: bool,

    /// The boolean indicator of whether to display version infos or not.
    ///
    /// * Optional value represented by the cli option
    ///     [`crate::constant::cli_options::VERSION`], and falling back to
    ///     `false` if not provided in cli args.
    /// * Has precedence over author option if multiple are given
    pub show_version: bool,

    /// The boolean indicator of whether to display author infos or not.
    ///
    /// * Optional value represented by the cli option
    ///     [`crate::constant::cli_options::AUTHOR`], and falling back to
    ///     `false` if not provided in cli args.
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

    /// Sets new value for `endpoint_uri` field.
    ///
    /// It needs to be called on struct instance and effectively mutates it.
    ///
    /// # Arguments
    ///
    /// * `endpoint_uri` - The new value to be assigned to `endpoint_uri`
    ///     field.
    ///
    /// # Returns
    ///
    /// The mutated borrowed instance.
    pub fn with_endpoint_uri(mut self, endpoint_uri: &str) -> Self {
        self.endpoint_uri = endpoint_uri.to_string();
        self
    }
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
    ///     error handling logic
    ///
    /// # Arguments
    ///
    /// * `args` - The CLI args to be parsed. Typically retrieved from
    ///     [`std::env::args_os`].
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
    ///     exit
    /// * Returned Args complies with expected constraints (see fields doc
    ///     in [`Args`] for more infos)
    ///
    /// # Arguments
    ///
    ///  * `args` - The CLI args to be parsed. Typically retrieved from
    ///     [`std::env::args_os`].
    ///
    /// # Returns
    ///
    /// A result containing an owned instance of [`Args`] if successful parsing,
    /// or a [`ProgramExit`] if any error or early exit occurred (e.g. version/
    /// author/help infos printing, invalid cli args...)
    fn try_parse(
        &self,
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
                use super::*;
                use crate::{ExitKind, constant};

                #[rstest]
                #[case("-V")]
                #[case("--version")]
                #[case("-V rust")]
                #[case("rust -V")]
                #[case("rust -s foo -V")]
                #[case("rust -e bar -V")]
                #[case("-aV")]
                fn it_parses_version_cli_option(#[case] cli_args: &str) {
                    let cli_args = parse_cli_args(cli_args);
                    let parsed_args = ClapArgsParser::new().try_parse(cli_args);

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
                #[case("rust -e bar -h")]
                #[case("-aVh")]
                fn it_parses_help_cli_option(#[case] cli_args: &str) {
                    let cli_args = parse_cli_args(cli_args);
                    let parsed_args = ClapArgsParser::new().try_parse(cli_args);

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
                #[case("rust -e bar -a")]
                fn it_parses_author_cli_option_preemptively(
                    #[case] cli_args: &str,
                ) {
                    let cli_args = parse_cli_args(cli_args);
                    let parsed_args = ClapArgsParser::new().try_parse(cli_args);

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
                    let parsed_args = ClapArgsParser::new().try_parse(cli_args);

                    let actual_result = parsed_args.as_ref().ok();
                    let expected_result = Args::default()
                        .with_template_names(make_string_vec(cli_options))
                        .with_server_url(constant::template_generator::BASE_URL)
                        .with_endpoint_uri(constant::template_generator::URI);
                    let expected_result = Some(&expected_result);

                    println!("{:?}", parsed_args);
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
                    let parsed_args = ClapArgsParser::new().try_parse(cli_args);

                    let actual_result = parsed_args.as_ref().ok();
                    let expected_result = Args::default()
                        .with_template_names(make_string_vec("rust"))
                        .with_server_url("https://test.com")
                        .with_endpoint_uri(constant::template_generator::URI);
                    let expected_result = Some(&expected_result);

                    assert!(actual_result.is_some());
                    assert_eq!(actual_result, expected_result);
                }

                #[rstest]
                #[case("rust -e /test/api")]
                #[case("rust --endpoint-uri /test/api")]
                fn it_parses_pos_args_with_endpoint_uri_cli_option(
                    #[case] cli_args: &str,
                ) {
                    let cli_args = parse_cli_args(cli_args);
                    let parsed_args = ClapArgsParser::new().try_parse(cli_args);

                    let actual_result = parsed_args.as_ref().ok();
                    let expected_result = Args::default()
                        .with_template_names(make_string_vec("rust"))
                        .with_server_url(constant::template_generator::BASE_URL)
                        .with_endpoint_uri("/test/api");
                    let expected_result = Some(&expected_result);

                    assert!(actual_result.is_some());
                    assert_eq!(actual_result, expected_result);
                }
            }

            mod failure {
                use super::*;
                use crate::{ExitKind, constant};

                #[test]
                fn it_fails_parsing_when_no_pos_args_given() {
                    let cli_args = parse_cli_args("");
                    let parsed_args = ClapArgsParser::new().try_parse(cli_args);

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
                    let parsed_args = ClapArgsParser::new().try_parse(cli_args);

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
                fn it_fails_parsing_when_whitespaces_in_pos_args() {
                    let cli_args = vec![
                        OsString::from(env!("CARGO_PKG_NAME")),
                        OsString::from("r "),
                    ];
                    let parsed_args = ClapArgsParser::new().try_parse(cli_args);

                    let actual_error = parsed_args.as_ref().err();
                    let expected_error = ProgramExit {
                        message: load_expectation_file_as_string(
                            "whitespace_pos_args_error",
                        ),
                        exit_status: constant::exit_status::GENERIC,

                        styled_message: Some(load_expectation_file_as_string(
                            "ansi_whitespace_pos_args_error",
                        )),
                        kind: ExitKind::Error,
                    };
                    let expected_error = Some(&expected_error);

                    assert!(actual_error.is_some());
                    assert_eq!(actual_error, expected_error);
                }

                #[test]
                fn it_fails_parsing_when_commas_and_whitespaces_in_pos_args() {
                    let cli_args = vec![
                        OsString::from(env!("CARGO_PKG_NAME")),
                        OsString::from("r ,"),
                    ];
                    let parsed_args = ClapArgsParser::new().try_parse(cli_args);

                    let actual_error = parsed_args.as_ref().err();
                    let expected_error = ProgramExit {
                        message: load_expectation_file_as_string(
                            "comma_whitespace_pos_args_error",
                        ),
                        exit_status: constant::exit_status::GENERIC,

                        styled_message: Some(load_expectation_file_as_string(
                            "ansi_comma_whitespace_pos_args_error",
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
                    let parsed_args = ClapArgsParser::new().try_parse(cli_args);

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
                fn it_fails_parsing_when_endpoint_uri_but_no_pos_args() {
                    let cli_args = parse_cli_args("-e /test/api");
                    let parsed_args = ClapArgsParser::new().try_parse(cli_args);

                    let actual_error = parsed_args.as_ref().err();
                    let expected_error = ProgramExit {
                        message: load_expectation_file_as_string(
                            "endpoint_uri_no_pos_args_error",
                        ),
                        exit_status: constant::exit_status::GENERIC,

                        styled_message: Some(load_expectation_file_as_string(
                            "ansi_endpoint_uri_no_pos_args_error",
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
                    let parsed_args = ClapArgsParser::new().try_parse(cli_args);

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
