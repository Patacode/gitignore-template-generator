use clap::{Arg, ArgAction};

use super::ClapArg;
use crate::constant;

pub struct VersionClapArg;

impl ClapArg for VersionClapArg {
    fn build() -> Arg {
        Arg::new("version")
            .id("VERSION")
            .short(constant::cli_options::VERSION.short)
            .long(constant::cli_options::VERSION.long)
            .help(constant::help_messages::VERSION)
            .action(ArgAction::SetTrue)
    }
}
