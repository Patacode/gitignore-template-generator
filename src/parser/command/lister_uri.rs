use clap::{Arg, ArgMatches};

use super::ClapArg;
use crate::{
    constant, helper,
    validator::{CliArgsValidator, DefaultCliArgsValidator},
};

pub struct ListerUriClapArg;

impl ClapArg<String> for ListerUriClapArg {
    fn build() -> Arg {
        Arg::new("lister_uri")
            .id("LISTER_URI")
            .short(helper::to_char(constant::cli_options::LISTER_URI.short))
            .long(constant::cli_options::LISTER_URI.long)
            .help(constant::help_messages::LISTER_URI)
            .value_parser(DefaultCliArgsValidator::is_starting_with_slash)
            .default_value(constant::template_manager::LISTER_URI)
    }

    fn from_arg_matches(arg_matches: &ArgMatches) -> String {
        arg_matches
            .get_one::<String>("LISTER_URI")
            .unwrap()
            .to_string()
    }
}
