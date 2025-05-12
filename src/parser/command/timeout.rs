use clap::Arg;

use super::ClapArg;
use crate::constant;

pub struct TimeoutClapArg;

impl ClapArg for TimeoutClapArg {
    fn build() -> Arg {
        Arg::new("timeout")
            .id("TIMEOUT")
            .short(constant::cli_options::TIMEOUT.short)
            .long(constant::cli_options::TIMEOUT.long)
            .help(constant::help_messages::TIMEOUT)
            .value_parser(clap::value_parser!(u64))
            .default_value(constant::template_manager::TIMEOUT)
    }
}
