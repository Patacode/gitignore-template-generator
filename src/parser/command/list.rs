use clap::{Arg, ArgAction};

use super::ClapArg;
use crate::{constant, helper};

pub struct ListClapArg;

impl ClapArg for ListClapArg {
    fn build() -> Arg {
        Arg::new("list")
            .id("LIST")
            .short(helper::to_char(constant::cli_options::LIST.short))
            .long(constant::cli_options::LIST.long)
            .help(constant::help_messages::LIST)
            .action(ArgAction::SetTrue)
    }
}
