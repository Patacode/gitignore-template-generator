use clap::{Arg, ArgAction};

use super::ClapArg;
use crate::constant;

pub struct HelpClapArg;

impl ClapArg for HelpClapArg {
    fn build() -> Arg {
        Arg::new("help")
            .id("HELP")
            .short(constant::cli_options::HELP.short)
            .long(constant::cli_options::HELP.long)
            .help(constant::help_messages::HELP)
            .action(ArgAction::SetTrue)
    }
}
