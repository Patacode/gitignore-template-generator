use clap::{Arg, ArgAction};

use crate::constant;

use super::ClapArg;

pub struct AuthorClapArg;

impl ClapArg for AuthorClapArg {
    fn build() -> Arg {
        Arg::new("author")
            .id("AUTHOR")
            .short(constant::cli_options::AUTHOR.short)
            .long(constant::cli_options::AUTHOR.long)
            .help(constant::help_messages::AUTHOR)
            .action(ArgAction::SetTrue)
    }
}
