use clap::Arg;

use super::ClapArg;
use crate::{
    constant,
    validator::{CliArgsValidator, DefaultCliArgsValidator},
};

pub struct GeneratorUriClapArg;

impl ClapArg for GeneratorUriClapArg {
    fn build() -> Arg {
        Arg::new("generator_uri")
            .id("GENERATOR_URI")
            .short(constant::cli_options::GENERATOR_URI.short)
            .long(constant::cli_options::GENERATOR_URI.long)
            .help(constant::help_messages::GENERATOR_URI)
            .value_parser(DefaultCliArgsValidator::is_starting_with_slash)
            .default_value(constant::template_manager::GENERATOR_URI)
    }
}
