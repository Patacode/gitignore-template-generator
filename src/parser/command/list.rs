use clap::{Arg, ArgAction, ArgMatches};

use super::ClapArg;
use crate::{
    constant,
    helper::{DefaultUtils, Utils},
};

pub struct ListClapArg;

impl ClapArg<bool> for ListClapArg {
    fn build() -> Arg {
        Arg::new("list")
            .id("LIST")
            .short(DefaultUtils::to_char(constant::cli_options::LIST.short))
            .long(constant::cli_options::LIST.long)
            .help(constant::help_messages::LIST)
            .action(ArgAction::SetTrue)
    }

    fn from_arg_matches(arg_matches: &ArgMatches) -> bool {
        arg_matches.get_flag("LIST")
    }
}
