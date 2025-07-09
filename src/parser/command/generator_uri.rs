use clap::{Arg, ArgMatches};

use super::ClapArg;
use crate::{
    constant,
    helper::{DefaultUtils, Utils},
    validator::{CliArgsValidator, DefaultCliArgsValidator},
};

pub struct GeneratorUriClapArg;

impl ClapArg<String> for GeneratorUriClapArg {
    fn build() -> Arg {
        Arg::new("generator_uri")
            .id("GENERATOR_URI")
            .short(DefaultUtils::to_char(
                constant::cli_options::GENERATOR_URI.short,
            ))
            .long(constant::cli_options::GENERATOR_URI.long)
            .help(constant::help_messages::GENERATOR_URI)
            .value_parser(DefaultCliArgsValidator::is_starting_with_slash)
            .default_value(constant::template_manager::GENERATOR_URI)
    }

    fn from_arg_matches(arg_matches: &ArgMatches) -> String {
        arg_matches
            .get_one::<String>("GENERATOR_URI")
            .unwrap()
            .to_string()
    }
}
