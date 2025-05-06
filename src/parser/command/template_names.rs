use clap::Arg;

use super::ClapArg;
use crate::{
    constant,
    validator::{CliArgsValidator, DefaultCliArgsValidator},
};

pub struct TemplateNamesClapArg;

impl ClapArg for TemplateNamesClapArg {
    fn build() -> Arg {
        Arg::new("template_names")
            .id("TEMPLATE_NAMES")
            .help(constant::help_messages::TEMPLATE_NAMES)
            .required_unless_present_any(["AUTHOR", "VERSION", "HELP", "LIST"])
            .value_parser(DefaultCliArgsValidator::has_valid_template_name)
            .num_args(1..)
    }
}
