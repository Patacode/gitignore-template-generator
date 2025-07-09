use clap::{Arg, ArgAction, ArgMatches};

use super::ClapArg;
use crate::{
    constant,
    helper::{DefaultUtils, Utils},
};

pub struct CheckClapArg;

impl ClapArg<bool> for CheckClapArg {
    fn build() -> Arg {
        Arg::new("check")
            .id("CHECK")
            .short(DefaultUtils::to_char(constant::cli_options::CHECK.short))
            .long(constant::cli_options::CHECK.long)
            .help(constant::help_messages::CHECK)
            .action(ArgAction::SetTrue)
    }

    fn from_arg_matches(arg_matches: &ArgMatches) -> bool {
        arg_matches.get_flag("CHECK")
    }
}
