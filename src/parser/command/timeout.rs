use clap::{
    Arg,
    builder::{ArgPredicate, OsStr},
};

use super::ClapArg;
use crate::{constant, helper};

pub struct TimeoutClapArg;

impl ClapArg for TimeoutClapArg {
    fn build() -> Arg {
        Arg::new("timeout")
            .id("TIMEOUT")
            .short(helper::to_char(constant::cli_options::TIMEOUT.short))
            .long(constant::cli_options::TIMEOUT.long)
            .help(format!(
                "{} [default: {}s/{}ms]",
                constant::help_messages::TIMEOUT,
                constant::template_manager::TIMEOUT,
                constant::template_manager::TIMEOUT_MILLISECOND
            ))
            .value_parser(clap::value_parser!(u64))
            .default_value_if(
                "TIMEOUT_UNIT",
                ArgPredicate::Equals(OsStr::from(
                    constant::template_manager::TIMEOUT_MILLISECOND_UNIT,
                )),
                constant::template_manager::TIMEOUT_MILLISECOND,
            )
    }
}
