use clap::Arg;

use crate::{
    constant,
    validator::{CliArgsValidator, DefaultCliArgsValidator},
};

use super::ClapArg;

pub struct TemplateNamesClapArg;

impl ClapArg for TemplateNamesClapArg {
    fn build() -> Arg {
        Arg::new("template_names")
            .id("TEMPLATE_NAMES")
            .help(constant::help_messages::TEMPLATE_NAMES)
            .required_unless_present_any(["AUTHOR", "VERSION", "HELP"])
            .value_parser(DefaultCliArgsValidator::has_valid_template_name)
            .num_args(1..)
    }
}
