use clap::Parser;

use crate::{
    constant,
    validator::{CliArgsValidator, DefaultCliArgsValidator},
};

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
    #[arg(
        required_unless_present_any = vec!["author", "version", "help"],
        value_parser = DefaultCliArgsValidator::has_no_commas,
        help = constant::help_messages::TEMPLATE_NAMES
    )]
    pub template_names: Vec<String>,

    #[arg(
        short = constant::cli_options::SERVER_URL.short,
        long = constant::cli_options::SERVER_URL.long,
        help = constant::help_messages::SERVER_URL,
        default_value = constant::template_generator::BASE_URL
    )]
    pub server_url: String,

    #[arg(
        id = "help",
        short = constant::cli_options::HELP.short,
        long = constant::cli_options::HELP.long,
        action = clap::ArgAction::SetTrue,
        help = constant::help_messages::HELP
    )]
    pub show_help: bool,

    #[arg(
        id = "version",
        short = constant::cli_options::VERSION.short,
        long = constant::cli_options::VERSION.long,
        action = clap::ArgAction::SetTrue,
        help = constant::help_messages::VERSION
    )]
    pub show_version: bool,

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
    pub fn with_template_names(mut self, template_names: Vec<String>) -> Self {
        self.template_names = template_names;
        self
    }

    pub fn with_server_url(mut self, server_url: &str) -> Self {
        self.server_url = server_url.to_string();
        self
    }
}
