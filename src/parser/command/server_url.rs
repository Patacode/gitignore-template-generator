use clap::{Arg, ArgMatches};

use super::ClapArg;
use crate::{
    constant, helper,
    validator::{CliArgsValidator, DefaultCliArgsValidator},
};

pub struct ServerUrlClapArg;

impl ClapArg<String> for ServerUrlClapArg {
    fn build() -> Arg {
        Arg::new("server_url")
            .id("SERVER_URL")
            .short(helper::to_char(constant::cli_options::SERVER_URL.short))
            .long(constant::cli_options::SERVER_URL.long)
            .help(constant::help_messages::SERVER_URL)
            .value_parser(DefaultCliArgsValidator::is_valid_url)
            .default_value(constant::template_manager::BASE_URL)
    }

    fn from_arg_matches(arg_matches: &ArgMatches) -> String {
        arg_matches
            .get_one::<String>("SERVER_URL")
            .unwrap()
            .to_string()
    }
}
