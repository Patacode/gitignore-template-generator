use clap::Arg;

use super::ClapArg;
use crate::{
    constant,
    validator::{CliArgsValidator, DefaultCliArgsValidator},
};

pub struct TemplateNamesClapArg;

impl ClapArg<Vec<String>> for TemplateNamesClapArg {
    fn build() -> Arg {
        Arg::new("template_names")
            .id("TEMPLATE_NAMES")
            .help(constant::help_messages::TEMPLATE_NAMES)
            .required_unless_present_any(["AUTHOR", "VERSION", "HELP", "LIST"])
            .value_parser(DefaultCliArgsValidator::is_valid_template_name)
            .num_args(1..)
    }

    fn from_arg_matches(arg_matches: &clap::ArgMatches) -> Vec<String> {
        arg_matches
            .get_many::<String>("TEMPLATE_NAMES")
            .map(|vals| vals.cloned().collect())
            .unwrap_or_default()
    }
}
