use clap::Arg;

use super::ClapArg;
use crate::constant;

pub struct ListerUriClapArg;

impl ClapArg for ListerUriClapArg {
    fn build() -> Arg {
        Arg::new("lister_uri")
            .id("LISTER_URI")
            .short(constant::cli_options::LISTER_URI.short)
            .long(constant::cli_options::LISTER_URI.long)
            .help(constant::help_messages::LISTER_URI)
            .default_value(constant::template_manager::LISTER_URI)
    }
}
