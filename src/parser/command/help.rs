use clap::{Arg, ArgAction, ArgMatches};

use super::ClapArg;
use crate::{constant, helper};

pub struct HelpClapArg;

impl ClapArg<bool> for HelpClapArg {
    fn build() -> Arg {
        Arg::new("help")
            .id("HELP")
            .short(helper::to_char(constant::cli_options::HELP.short))
            .long(constant::cli_options::HELP.long)
            .help(constant::help_messages::HELP)
            .action(ArgAction::SetTrue)
    }

    fn from_arg_matches(arg_matches: &ArgMatches) -> bool {
        arg_matches.get_flag("HELP")
    }
}
