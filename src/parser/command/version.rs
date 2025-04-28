use clap::{Arg, ArgAction};

use crate::constant;

use super::ClapArg;

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
