//! Components used to configure cli args parsing

use clap::Parser;

use crate::{
    constant,
    validator::{CliArgsValidator, DefaultCliArgsValidator},
};

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
    /// * `template_names` - The new value to be assigned to `template_names`
    /// field.
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
    /// * `server_url` - The new value to be assigned to `server_url`
    /// field.
    /// 
    /// # Returns
    /// 
    /// The mutated borrowed instance.
    pub fn with_server_url(mut self, server_url: &str) -> Self {
        self.server_url = server_url.to_string();
        self
    }
}
