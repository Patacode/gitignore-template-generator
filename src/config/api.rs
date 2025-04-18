use clap::Parser;

pub use crate::config::impls::DefaultArgsParser;
use crate::{
    constant,
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
}
