use clap::{
    Arg, ArgMatches,
    builder::{ArgPredicate, OsStr},
};

use super::ClapArg;
use crate::{
    constant,
    helper::{DefaultUtils, Utils},
};

pub struct TimeoutClapArg;

impl ClapArg<u64> for TimeoutClapArg {
    fn build() -> Arg {
        Arg::new("timeout")
            .id("TIMEOUT")
            .short(DefaultUtils::to_char(constant::cli_options::TIMEOUT.short))
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

    fn from_arg_matches(arg_matches: &ArgMatches) -> u64 {
        match arg_matches.get_one::<u64>("TIMEOUT") {
            Some(timeout) => *timeout,
            None => constant::template_manager::TIMEOUT_INT,
        }
    }
}
