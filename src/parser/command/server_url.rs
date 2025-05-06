use clap::Arg;

use super::ClapArg;
use crate::constant;

pub struct ServerUrlClapArg;

impl ClapArg for ServerUrlClapArg {
    fn build() -> Arg {
        Arg::new("server_url")
            .id("SERVER_URL")
            .short(constant::cli_options::SERVER_URL.short)
            .long(constant::cli_options::SERVER_URL.long)
            .help(constant::help_messages::SERVER_URL)
            .default_value(constant::template_manager::BASE_URL)
    }
}
