use std::ffi::OsString;

use clap::Parser;

pub use crate::config::impls::DefaultArgsParser;
use crate::{
    constant,
    http_client::ProgramError,
    validator::{CliArgsValidator, DefaultCliArgsValidator},
};

#[derive(Parser, Debug, PartialEq)]
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
pub struct Args {
    #[arg(
        required_unless_present = "author",
        value_parser = DefaultCliArgsValidator::has_no_commas,
        help = constant::help_messages::TEMPLATE_NAMES
    )]
    pub template_names: Vec<String>,

    #[arg(
        id = "author",
        short = constant::cli_options::AUTHOR.short,
        long = constant::cli_options::AUTHOR.long,
        action = clap::ArgAction::SetTrue,
        help = constant::help_messages::AUTHOR
    )]
    pub show_author: bool,

    #[arg(
        short = constant::cli_options::SERVER_URL.short,
        long = constant::cli_options::SERVER_URL.long,
        help = constant::help_messages::SERVER_URL,
        default_value = constant::template_generator::BASE_URL
    )]
    pub server_url: String,
}

pub trait ArgsParser {
    fn parse() -> Args;
    fn try_parse(
        args: impl IntoIterator<Item = OsString>,
    ) -> Result<Args, ProgramError>;
}

#[cfg(test)]
mod tests {
    use rstest::*;

    use super::*;
    use crate::helper::*;

    mod default_args_parser {
        use super::*;

        mod parse {
            use super::*;

            mod success {
                use super::*;

                #[rstest]
                #[case("-V")]
                #[case("--version")]
                fn it_parses_version_cli_option(#[case] cli_args: &str) {
                    let cli_args = parse_cli_args(cli_args);
                    let parsed_args = DefaultArgsParser::try_parse(cli_args);

                    let actual_error = parsed_args.as_ref().err();
                    let expected_error = ProgramError {
                        message: format!(
                            "{} {}\n",
                            env!("CARGO_PKG_NAME"),
                            env!("CARGO_PKG_VERSION")
                        ),
                        exit_status: 0,
                    };
                    let expected_error = Some(&expected_error);

                    assert!(actual_error.is_some());
                    assert_eq!(actual_error, expected_error);
                }

                #[rstest]
                #[case("-h")]
                #[case("--help")]
                fn it_parses_help_cli_option(#[case] cli_args: &str) {
                    let cli_args = parse_cli_args(cli_args);
                    let parsed_args = DefaultArgsParser::try_parse(cli_args);

                    let actual_error = parsed_args.as_ref().err();
                    let expected_error = ProgramError {
                        message: get_help_message(),
                        exit_status: 0,
                    };
                    let expected_error = Some(&expected_error);

                    assert!(actual_error.is_some());
                    assert_eq!(actual_error, expected_error);
                }

                #[rstest]
                #[case("-a")]
                #[case("--author")]
                fn it_parses_author_cli_option(#[case] cli_args: &str) {
                    let cli_args = parse_cli_args(cli_args);
                    let parsed_args = DefaultArgsParser::try_parse(cli_args);

                    let actual_error = parsed_args.as_ref().err();
                    let expected_error = ProgramError {
                        message: env!("CARGO_PKG_AUTHORS").to_string(),
                        exit_status: 0,
                    };
                    let expected_error = Some(&expected_error);

                    assert!(actual_error.is_some());
                    assert_eq!(actual_error, expected_error);
                }

                #[rstest]
                #[case("rust")]
                #[case("rust python node")]
                fn it_parses_pos_args_cli_option(#[case] cli_options: &str) {
                    let cli_args = parse_cli_args(cli_options);
                    let parsed_args = DefaultArgsParser::try_parse(cli_args);

                    println!("{:?}", parsed_args);
                    let actual_result = parsed_args.as_ref().ok();
                    let expected_result = Args {
                        template_names: make_string_vec(cli_options),
                        show_author: false,
                        server_url: constant::template_generator::BASE_URL
                            .to_string(),
                    };
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

                    println!("{:?}", parsed_args);
                    let actual_result = parsed_args.as_ref().ok();
                    let expected_result = Args {
                        template_names: make_string_vec("rust"),
                        show_author: false,
                        server_url: "https://test.com".to_string(),
                    };
                    let expected_result = Some(&expected_result);

                    assert!(actual_result.is_some());
                    assert_eq!(actual_result, expected_result);
                }
            }
        }
    }
}
