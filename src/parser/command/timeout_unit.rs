use clap::{Arg, builder::EnumValueParser};

use super::ClapArg;
use crate::{constant, helper, parser::TimeoutUnit};

pub struct TimeoutUnitClapArg;

impl ClapArg for TimeoutUnitClapArg {
    fn build() -> Arg {
        Arg::new("timeout_unit")
            .id("TIMEOUT_UNIT")
            .short(helper::to_char(constant::cli_options::TIMEOUT_UNIT.short))
            .long(constant::cli_options::TIMEOUT_UNIT.long)
            .help(constant::help_messages::TIMEOUT_UNIT)
            .value_parser(EnumValueParser::<TimeoutUnit>::new())
            .default_value(constant::template_manager::TIMEOUT_UNIT)
    }
}
