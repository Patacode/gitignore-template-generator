use std::ffi::OsString;

use rstest::*;

use super::*;
use crate::{
    constant,
    core::{ExitKind, ProgramExit},
    helper::TimeoutUnit,
    test_helper::{DefaultTestUtils, TestUtils},
};

mod default_args_parser {
    use super::*;

    mod try_parse {
        use super::*;

        mod success {
            use super::*;

            #[rstest]
            #[case("-V")]
            #[case("--version")]
            #[case("-V rust")]
            #[case("rust -V")]
            #[case("rust -s https://foo -V")]
            #[case("rust -g /bar -V")]
            #[case("rust -i /bar -V")]
            #[case("rust -c -V")]
            #[case("rust -t 5 -V")]
            #[case("rust -u second -V")]
            #[case("-aV")]
            #[case("rust -l -V")]
            fn it_parses_version_cli_option(#[case] cli_args: &str) {
                let cli_args = DefaultTestUtils::parse_and_map_cli_args(
                    cli_args,
                    DefaultTestUtils::to_os_string,
                );
                let parsed_args = ClapArgsParser::new().try_parse(cli_args);

                let actual_error = parsed_args.as_ref().err();
                let expected_error = ProgramExit {
                    message: format!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION")),
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
            #[case("rust -s https://foo -h")]
            #[case("rust -g /bar -h")]
            #[case("rust -i /bar -h")]
            #[case("rust -c -h")]
            #[case("rust -t 5 -h")]
            #[case("rust -u second -h")]
            #[case("-aVh")]
            #[case("rust -l -h")]
            fn it_parses_help_cli_option(#[case] cli_args: &str) {
                let cli_args = DefaultTestUtils::parse_and_map_cli_args(
                    cli_args,
                    DefaultTestUtils::to_os_string,
                );
                let parsed_args = ClapArgsParser::new().try_parse(cli_args);

                let actual_error = parsed_args.as_ref().err();
                let expected_error = ProgramExit {
                    message: DefaultTestUtils::get_help_message(),
                    exit_status: 0,
                    styled_message: Some(DefaultTestUtils::get_ansi_help_message()),
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
            #[case("rust -s https://foo -a")]
            #[case("rust -g /bar -a")]
            #[case("rust -i /bar -a")]
            #[case("rust -c -a")]
            #[case("rust -t 5 -a")]
            #[case("rust -u second -a")]
            #[case("rust -l -a")]
            fn it_parses_author_cli_option_preemptively(#[case] cli_args: &str) {
                let cli_args = DefaultTestUtils::parse_and_map_cli_args(
                    cli_args,
                    DefaultTestUtils::to_os_string,
                );
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
            fn it_parses_pos_args_without_server_url_cli_option(#[case] cli_options: &str) {
                let cli_args = DefaultTestUtils::parse_and_map_cli_args(
                    cli_options,
                    DefaultTestUtils::to_os_string,
                );
                let parsed_args = ClapArgsParser::new().try_parse(cli_args);

                let actual_result = parsed_args.as_ref().ok();
                let expected_result =
                    Args::new().with_template_names(DefaultTestUtils::to_string_list(cli_options));
                let expected_result = Some(&expected_result);

                println!("{:?}", parsed_args);
                assert!(actual_result.is_some());
                assert_eq!(actual_result, expected_result);
            }

            #[rstest]
            #[case("rust -s https://test.com")]
            #[case("rust --server-url https://test.com")]
            fn it_parses_pos_args_with_server_url_cli_option(#[case] cli_args: &str) {
                let cli_args = DefaultTestUtils::parse_and_map_cli_args(
                    cli_args,
                    DefaultTestUtils::to_os_string,
                );
                let parsed_args = ClapArgsParser::new().try_parse(cli_args);

                let actual_result = parsed_args.as_ref().ok();
                let expected_result = Args::new()
                    .with_template_names(DefaultTestUtils::to_string_list("rust"))
                    .with_server_url("https://test.com");
                let expected_result = Some(&expected_result);

                assert!(actual_result.is_some());
                assert_eq!(actual_result, expected_result);
            }

            #[rstest]
            #[case("rust -g /test/api")]
            #[case("rust --generator-uri /test/api")]
            fn it_parses_pos_args_with_generator_uri_cli_option(#[case] cli_args: &str) {
                let cli_args = DefaultTestUtils::parse_and_map_cli_args(
                    cli_args,
                    DefaultTestUtils::to_os_string,
                );
                let parsed_args = ClapArgsParser::new().try_parse(cli_args);

                let actual_result = parsed_args.as_ref().ok();
                let expected_result = Args::new()
                    .with_template_names(DefaultTestUtils::to_string_list("rust"))
                    .with_server_url(constant::template_manager::BASE_URL)
                    .with_generator_uri("/test/api");
                let expected_result = Some(&expected_result);

                assert!(actual_result.is_some());
                assert_eq!(actual_result, expected_result);
            }

            #[rstest]
            #[case("rust -i /test/api")]
            #[case("rust --lister-uri /test/api")]
            fn it_parses_pos_args_with_lister_uri_cli_option(#[case] cli_args: &str) {
                let cli_args = DefaultTestUtils::parse_and_map_cli_args(
                    cli_args,
                    DefaultTestUtils::to_os_string,
                );
                let parsed_args = ClapArgsParser::new().try_parse(cli_args);

                let actual_result = parsed_args.as_ref().ok();
                let expected_result = Args::new()
                    .with_template_names(DefaultTestUtils::to_string_list("rust"))
                    .with_lister_uri("/test/api");
                let expected_result = Some(&expected_result);

                assert!(actual_result.is_some());
                assert_eq!(actual_result, expected_result);
            }

            #[rstest]
            #[case("-l", "")]
            #[case("--list", "")]
            #[case("rust --list", "rust")]
            #[case("rust python --list", "rust python")]
            fn it_parses_list_cli_option(#[case] cli_args: &str, #[case] template_names: &str) {
                let cli_args = DefaultTestUtils::parse_and_map_cli_args(
                    cli_args,
                    DefaultTestUtils::to_os_string,
                );
                let parsed_args = ClapArgsParser::new().try_parse(cli_args);

                let actual_result = parsed_args.as_ref().ok();
                let expected_result = Args::new()
                    .with_template_names(DefaultTestUtils::to_string_list(template_names))
                    .with_show_list(true);
                let expected_result = Some(&expected_result);

                assert!(actual_result.is_some());
                assert_eq!(actual_result, expected_result);
            }

            #[rstest]
            #[case("rust python -c")]
            #[case("rust python --check")]
            fn it_parses_check_option(#[case] cli_args: &str) {
                let cli_args = DefaultTestUtils::parse_and_map_cli_args(
                    cli_args,
                    DefaultTestUtils::to_os_string,
                );
                let parsed_args = ClapArgsParser::new().try_parse(cli_args);

                let actual_result = parsed_args.as_ref().ok();
                let expected_result = Args::new()
                    .with_template_names(DefaultTestUtils::to_string_list("rust python"))
                    .with_check_template_names(true);
                let expected_result = Some(&expected_result);

                assert!(actual_result.is_some());
                assert_eq!(actual_result, expected_result);
            }

            #[rstest]
            #[case("rust python -t 5")]
            #[case("rust python --timeout 5")]
            fn it_parses_timeout_option(#[case] cli_args: &str) {
                let cli_args = DefaultTestUtils::parse_and_map_cli_args(
                    cli_args,
                    DefaultTestUtils::to_os_string,
                );
                let parsed_args = ClapArgsParser::new().try_parse(cli_args);

                let actual_result = parsed_args.as_ref().ok();
                let expected_result = Args::new()
                    .with_template_names(DefaultTestUtils::to_string_list("rust python"))
                    .with_timeout(5);
                let expected_result = Some(&expected_result);

                assert!(actual_result.is_some());
                assert_eq!(actual_result, expected_result);
            }

            #[rstest]
            #[case("rust python -u second", TimeoutUnit::SECOND)]
            #[case("rust python --timeout-unit millisecond", TimeoutUnit::MILLISECOND)]
            fn it_parses_timeout_unit_option(#[case] cli_args: &str, #[case] unit: TimeoutUnit) {
                let cli_args = DefaultTestUtils::parse_and_map_cli_args(
                    cli_args,
                    DefaultTestUtils::to_os_string,
                );
                let parsed_args = ClapArgsParser::new().try_parse(cli_args);

                let actual_result = parsed_args.as_ref().ok();
                let expected_result = Args::new()
                    .with_template_names(DefaultTestUtils::to_string_list("rust python"))
                    .with_timeout_unit(unit)
                    .with_timeout(if unit == TimeoutUnit::MILLISECOND {
                        constant::template_manager::TIMEOUT_MILLISECOND_INT
                    } else {
                        constant::template_manager::TIMEOUT_INT
                    });
                let expected_result = Some(&expected_result);

                assert!(actual_result.is_some());
                assert_eq!(actual_result, expected_result);
            }
        }

        mod failure {
            use super::*;

            #[test]
            fn it_fails_parsing_when_no_pos_args_given() {
                let cli_args =
                    DefaultTestUtils::parse_and_map_cli_args("", DefaultTestUtils::to_os_string);
                let parsed_args = ClapArgsParser::new().try_parse(cli_args);

                let actual_error = parsed_args.as_ref().err();
                let expected_error = ProgramExit {
                    message: DefaultTestUtils::load_expectation_file("no_pos_args_error"),
                    exit_status: constant::exit_status::GENERIC,

                    styled_message: Some(DefaultTestUtils::load_expectation_file(
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
                let cli_args = DefaultTestUtils::parse_and_map_cli_args(
                    "python,java",
                    DefaultTestUtils::to_os_string,
                );
                let parsed_args = ClapArgsParser::new().try_parse(cli_args);

                let actual_error = parsed_args.as_ref().err();
                let expected_error = ProgramExit {
                    message: DefaultTestUtils::load_expectation_file("comma_pos_args_error"),
                    exit_status: constant::exit_status::GENERIC,

                    styled_message: Some(DefaultTestUtils::load_expectation_file(
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
                let cli_args = vec![OsString::from(env!("CARGO_PKG_NAME")), OsString::from("r ")];
                let parsed_args = ClapArgsParser::new().try_parse(cli_args);

                let actual_error = parsed_args.as_ref().err();
                let expected_error = ProgramExit {
                    message: DefaultTestUtils::load_expectation_file("whitespace_pos_args_error"),
                    exit_status: constant::exit_status::GENERIC,

                    styled_message: Some(DefaultTestUtils::load_expectation_file(
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
                    message: DefaultTestUtils::load_expectation_file(
                        "comma_whitespace_pos_args_error",
                    ),
                    exit_status: constant::exit_status::GENERIC,

                    styled_message: Some(DefaultTestUtils::load_expectation_file(
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
                let cli_args = DefaultTestUtils::parse_and_map_cli_args(
                    "-s https://test.com",
                    DefaultTestUtils::to_os_string,
                );
                let parsed_args = ClapArgsParser::new().try_parse(cli_args);

                let actual_error = parsed_args.as_ref().err();
                let expected_error = ProgramExit {
                    message: DefaultTestUtils::load_expectation_file(
                        "server_url_no_pos_args_error",
                    ),
                    exit_status: constant::exit_status::GENERIC,

                    styled_message: Some(DefaultTestUtils::load_expectation_file(
                        "ansi_server_url_no_pos_args_error",
                    )),
                    kind: ExitKind::Error,
                };
                let expected_error = Some(&expected_error);

                assert!(actual_error.is_some());
                assert_eq!(actual_error, expected_error);
            }

            #[test]
            fn it_fails_parsing_when_generator_uri_but_no_pos_args() {
                let cli_args = DefaultTestUtils::parse_and_map_cli_args(
                    "-g /test/api",
                    DefaultTestUtils::to_os_string,
                );
                let parsed_args = ClapArgsParser::new().try_parse(cli_args);

                let actual_error = parsed_args.as_ref().err();
                let expected_error = ProgramExit {
                    message: DefaultTestUtils::load_expectation_file(
                        "generator_uri_no_pos_args_error",
                    ),
                    exit_status: constant::exit_status::GENERIC,

                    styled_message: Some(DefaultTestUtils::load_expectation_file(
                        "ansi_generator_uri_no_pos_args_error",
                    )),
                    kind: ExitKind::Error,
                };
                let expected_error = Some(&expected_error);

                assert!(actual_error.is_some());
                assert_eq!(actual_error, expected_error);
            }

            #[test]
            fn it_fails_parsing_when_lister_uri_but_no_pos_args() {
                let cli_args = DefaultTestUtils::parse_and_map_cli_args(
                    "-i /test/api",
                    DefaultTestUtils::to_os_string,
                );
                let parsed_args = ClapArgsParser::new().try_parse(cli_args);

                let actual_error = parsed_args.as_ref().err();
                let expected_error = ProgramExit {
                    message: DefaultTestUtils::load_expectation_file(
                        "lister_uri_no_pos_args_error",
                    ),
                    exit_status: constant::exit_status::GENERIC,

                    styled_message: Some(DefaultTestUtils::load_expectation_file(
                        "ansi_lister_uri_no_pos_args_error",
                    )),
                    kind: ExitKind::Error,
                };
                let expected_error = Some(&expected_error);

                assert!(actual_error.is_some());
                assert_eq!(actual_error, expected_error);
            }

            #[test]
            fn it_fails_parsing_when_check_option_but_no_pos_args() {
                let cli_args = DefaultTestUtils::parse_and_map_cli_args(
                    "--check",
                    DefaultTestUtils::to_os_string,
                );
                let parsed_args = ClapArgsParser::new().try_parse(cli_args);

                let actual_error = parsed_args.as_ref().err();
                let expected_error = ProgramExit {
                    message: DefaultTestUtils::load_expectation_file(
                        "check_option_no_pos_args_error",
                    ),
                    exit_status: constant::exit_status::GENERIC,

                    styled_message: Some(DefaultTestUtils::load_expectation_file(
                        "ansi_check_option_no_pos_args_error",
                    )),
                    kind: ExitKind::Error,
                };
                let expected_error = Some(&expected_error);

                assert!(actual_error.is_some());
                assert_eq!(actual_error, expected_error);
            }

            #[test]
            fn it_fails_parsing_when_inexistent_cli_option() {
                let cli_args =
                    DefaultTestUtils::parse_and_map_cli_args("-x", DefaultTestUtils::to_os_string);
                let parsed_args = ClapArgsParser::new().try_parse(cli_args);

                let actual_error = parsed_args.as_ref().err();
                let expected_error = ProgramExit {
                    message: DefaultTestUtils::load_expectation_file("unexpected_argument_error"),
                    exit_status: constant::exit_status::GENERIC,
                    styled_message: Some(DefaultTestUtils::load_expectation_file(
                        "ansi_unexpected_argument_error",
                    )),
                    kind: ExitKind::Error,
                };
                let expected_error = Some(&expected_error);

                assert!(actual_error.is_some());
                assert_eq!(actual_error, expected_error);
            }

            #[test]
            fn it_fails_parsing_when_non_positive_integer_as_timeout() {
                let cli_args = DefaultTestUtils::parse_and_map_cli_args(
                    "-t x",
                    DefaultTestUtils::to_os_string,
                );
                let parsed_args = ClapArgsParser::new().try_parse(cli_args);

                let actual_error = parsed_args.as_ref().err();
                let expected_error = ProgramExit {
                    message: DefaultTestUtils::load_expectation_file("non_integer_timeout_error"),
                    exit_status: constant::exit_status::GENERIC,
                    styled_message: Some(DefaultTestUtils::load_expectation_file(
                        "ansi_non_integer_timeout_error",
                    )),
                    kind: ExitKind::Error,
                };
                let expected_error = Some(&expected_error);

                assert!(actual_error.is_some());
                assert_eq!(actual_error, expected_error);
            }

            #[test]
            fn it_fails_parsing_when_non_allowed_timeout_unit() {
                let cli_args = DefaultTestUtils::parse_and_map_cli_args(
                    "-u millisecon",
                    DefaultTestUtils::to_os_string,
                );
                let parsed_args = ClapArgsParser::new().try_parse(cli_args);

                let actual_error = parsed_args.as_ref().err();
                let expected_error = ProgramExit {
                    message: DefaultTestUtils::load_expectation_file(
                        "non_allowed_timeout_unit_error",
                    ),
                    exit_status: constant::exit_status::GENERIC,
                    styled_message: Some(DefaultTestUtils::load_expectation_file(
                        "ansi_non_allowed_timeout_unit_error",
                    )),
                    kind: ExitKind::Error,
                };
                let expected_error = Some(&expected_error);

                assert!(actual_error.is_some());
                assert_eq!(actual_error, expected_error);
            }

            #[rstest]
            #[case("-cc", "--check")]
            #[case("-g /bar -g /foo", "--generator-uri <GENERATOR_URI>")]
            #[case("-ll", "--list")]
            #[case("-i /bar -i /foo", "--lister-uri <LISTER_URI>")]
            #[case("-s https://foo.com -s https://bar.com", "--server-url <SERVER_URL>")]
            #[case("-t1 -t2", "--timeout <TIMEOUT>")]
            #[case("-u millisecond -u second", "--timeout-unit <TIMEOUT_UNIT>")]
            #[case("-hh", "--help")]
            #[case("-VV", "--version")]
            #[case("-aa", "--author")]
            fn it_fails_parsing_when_option_specified_multiple_times(
                #[case] cli_args: &str,
                #[case] option_name: &str,
            ) {
                let cli_args = DefaultTestUtils::parse_and_map_cli_args(
                    cli_args,
                    DefaultTestUtils::to_os_string,
                );
                let parsed_args = ClapArgsParser::new().try_parse(cli_args);

                let actual_error = parsed_args.as_ref().err();
                let expected_error = ProgramExit {
                    message: DefaultTestUtils::load_expectation_file("given_multiple_times_error")
                        .replace("{argument_name}", option_name),
                    exit_status: constant::exit_status::GENERIC,
                    styled_message: Some(
                        DefaultTestUtils::load_expectation_file("ansi_given_multiple_times_error")
                            .replace("{argument_name}", option_name),
                    ),
                    kind: ExitKind::Error,
                };
                let expected_error = Some(&expected_error);

                assert!(actual_error.is_some());
                assert_eq!(actual_error, expected_error);
            }

            #[rstest]
            #[case("--check=true", "--check")]
            #[case("--list=true", "--list")]
            #[case("--help=true", "--help")]
            #[case("--version=true", "--version")]
            #[case("--author=true", "--author")]
            fn it_fails_parsing_when_value_given_to_boolean_option(
                #[case] cli_args: &str,
                #[case] option_name: &str,
            ) {
                let cli_args = DefaultTestUtils::parse_and_map_cli_args(
                    cli_args,
                    DefaultTestUtils::to_os_string,
                );
                let parsed_args = ClapArgsParser::new().try_parse(cli_args);

                let actual_error = parsed_args.as_ref().err();
                let expected_error = ProgramExit {
                    message: DefaultTestUtils::load_expectation_file(
                        "boolean_option_with_value_error",
                    )
                    .replace("{argument_name}", option_name),
                    exit_status: constant::exit_status::GENERIC,
                    styled_message: Some(
                        DefaultTestUtils::load_expectation_file(
                            "ansi_boolean_option_with_value_error",
                        )
                        .replace("{argument_name}", option_name),
                    ),
                    kind: ExitKind::Error,
                };
                let expected_error = Some(&expected_error);

                assert!(actual_error.is_some());
                assert_eq!(actual_error, expected_error);
            }

            #[rstest]
            #[case("--lister-uri foo", "--lister-uri <LISTER_URI>")]
            #[case("--generator-uri foo", "--generator-uri <GENERATOR_URI>")]
            fn it_fails_parsing_when_uri_without_starting_slash(
                #[case] cli_args: &str,
                #[case] option_name: &str,
            ) {
                let cli_args = DefaultTestUtils::parse_and_map_cli_args(
                    cli_args,
                    DefaultTestUtils::to_os_string,
                );
                let parsed_args = ClapArgsParser::new().try_parse(cli_args);

                let actual_error = parsed_args.as_ref().err();
                let expected_error = ProgramExit {
                    message: DefaultTestUtils::load_expectation_file(
                        "uri_without_starting_slash_error",
                    )
                    .replace("{argument_name}", option_name),
                    exit_status: constant::exit_status::GENERIC,
                    styled_message: Some(
                        DefaultTestUtils::load_expectation_file(
                            "ansi_uri_without_starting_slash_error",
                        )
                        .replace("{argument_name}", option_name),
                    ),
                    kind: ExitKind::Error,
                };
                let expected_error = Some(&expected_error);

                assert!(actual_error.is_some());
                assert_eq!(actual_error, expected_error);
            }

            #[rstest]
            #[case("--server-url foo", "foo", "invalid_url_error")]
            #[case(
                "--server-url xyz:://foo.com",
                "xyz:://foo.com",
                "invalid_scheme_error"
            )]
            fn it_fails_parsing_when_invalid_url(
                #[case] cli_args: &str,
                #[case] invalid_value: &str,
                #[case] expectation_filename: &str,
            ) {
                let cli_args = DefaultTestUtils::parse_and_map_cli_args(
                    cli_args,
                    DefaultTestUtils::to_os_string,
                );
                let parsed_args = ClapArgsParser::new().try_parse(cli_args);

                let actual_error = parsed_args.as_ref().err();
                let expected_error = ProgramExit {
                    message: DefaultTestUtils::load_expectation_file(expectation_filename)
                        .replace("{input_value}", invalid_value),
                    exit_status: constant::exit_status::GENERIC,
                    styled_message: Some(
                        DefaultTestUtils::load_expectation_file(&format!(
                            "ansi_{expectation_filename}"
                        ))
                        .replace("{input_value}", invalid_value),
                    ),
                    kind: ExitKind::Error,
                };
                let expected_error = Some(&expected_error);

                assert!(actual_error.is_some());
                assert_eq!(actual_error, expected_error);
            }
        }
    }

    mod parse {
        use super::*;

        mod success {
            use super::*;

            #[test]
            fn it_parses_given_cli_options() {
                let cli_args = DefaultTestUtils::parse_and_map_cli_args(
                    "rust python -s https://test -g /foo -i /bar --check --list -t 6 -u millisecond",
                    DefaultTestUtils::to_os_string,
                );

                let actual_result = ClapArgsParser::new().parse(cli_args);
                let expected_result = Args::new()
                    .with_template_names(DefaultTestUtils::to_string_list("rust python"))
                    .with_server_url("https://test")
                    .with_generator_uri("/foo")
                    .with_lister_uri("/bar")
                    .with_check_template_names(true)
                    .with_show_list(true)
                    .with_timeout(6)
                    .with_timeout_unit(TimeoutUnit::MILLISECOND);

                assert_eq!(actual_result, expected_result);
            }
        }
    }
}
