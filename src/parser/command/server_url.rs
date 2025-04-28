use clap::Arg;

use crate::constant;

use super::ClapArg;

pub struct ServerUrlClapArg;

impl ClapArg for ServerUrlClapArg {
    fn build() -> Arg {
        Arg::new("server_url")
            .id("SERVER_URL")
            .short(constant::cli_options::SERVER_URL.short)
            .long(constant::cli_options::SERVER_URL.long)
            .help(constant::help_messages::SERVER_URL)
            .default_value(constant::template_generator::BASE_URL)
    }
}
