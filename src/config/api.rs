use std::ffi::OsString;

use clap::Parser;

pub use crate::config::impls::DefaultArgsParser;
use crate::{
    constant,
    http_client::ProgramError,
    validator::{CliArgsValidator, DefaultCliArgsValidator},
};

#[derive(Parser, Debug)]
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
    use super::*;

    mod default_args_parser {
        use super::*;

        mod parse {
            use super::*;

            mod success {
                use super::*;

                #[test]
                fn it_parses_version_cli_option() {
                    let cli_args =
                        vec![OsString::from("test"), OsString::from("-V")];

                    let parsed_args = DefaultArgsParser::try_parse(cli_args);

                    let actual_output =
                        &parsed_args.as_ref().err().unwrap().message;
                    let expected_output = format!(
                        "{} {}\n",
                        env!("CARGO_PKG_NAME"),
                        env!("CARGO_PKG_VERSION")
                    );

                    let actual_exit_status =
                        &parsed_args.as_ref().err().unwrap().exit_status;
                    let expected_exit_status = 0;

                    assert_eq!(actual_output, &expected_output);
                    assert_eq!(actual_exit_status, &expected_exit_status);
                }
            }
        }
    }
}
