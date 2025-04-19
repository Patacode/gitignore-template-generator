use clap::Parser;

pub use crate::config::impls::DefaultArgsParser;
use crate::{
    constant,
    validator::{CliArgsValidator, DefaultCliArgsValidator},
};

#[derive(Parser, Debug)]
#[command(version, author, long_about = None)]
#[command(about = "Generate templates for .gitignore files")]
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
        short = 'a',
        long = "author",
        action = clap::ArgAction::SetTrue,
        help = constant::help_messages::AUTHOR
    )]
    pub show_author: bool,

    #[arg(
        short = 's',
        long = "server-url",
        help = constant::help_messages::SERVER_URL,
        default_value = constant::template_generator::BASE_URL
    )]
    pub server_url: String,
}

pub trait ArgsParser {
    fn parse() -> Args;
}
