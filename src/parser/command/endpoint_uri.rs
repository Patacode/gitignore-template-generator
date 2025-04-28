use clap::Arg;

use crate::constant;

use super::ClapArg;

pub struct EndpointUriClapArg;

impl ClapArg for EndpointUriClapArg {
    fn build() -> Arg {
        Arg::new("endpoint_uri")
            .id("ENDPOINT_URI")
            .short(constant::cli_options::ENDPOINT_URI.short)
            .long(constant::cli_options::ENDPOINT_URI.long)
            .help(constant::help_messages::ENDPOINT_URI)
            .default_value(constant::template_generator::URI)
    }
}
