use clap::{Arg, ArgAction, ArgMatches};

use super::ClapArg;
use crate::{constant, helper};

pub struct VersionClapArg;

impl ClapArg<bool> for VersionClapArg {
    fn build() -> Arg {
        Arg::new("version")
            .id("VERSION")
            .short(helper::to_char(constant::cli_options::VERSION.short))
            .long(constant::cli_options::VERSION.long)
            .help(constant::help_messages::VERSION)
            .action(ArgAction::SetTrue)
    }

    fn from_arg_matches(arg_matches: &ArgMatches) -> bool {
        arg_matches.get_flag("VERSION")
    }
}
