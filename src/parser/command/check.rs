use clap::{Arg, ArgAction};

use super::ClapArg;
use crate::{constant, helper};

pub struct CheckClapArg;

impl ClapArg for CheckClapArg {
    fn build() -> Arg {
        Arg::new("check")
            .id("CHECK")
            .short(helper::to_char(constant::cli_options::CHECK.short))
            .long(constant::cli_options::CHECK.long)
            .help(constant::help_messages::CHECK)
            .action(ArgAction::SetTrue)
    }
}
