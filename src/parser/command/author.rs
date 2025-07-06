use clap::{Arg, ArgAction};

use super::ClapArg;
use crate::{constant, helper};

pub struct AuthorClapArg;

impl ClapArg<bool> for AuthorClapArg {
    fn build() -> Arg {
        Arg::new("author")
            .id("AUTHOR")
            .short(helper::to_char(constant::cli_options::AUTHOR.short))
            .long(constant::cli_options::AUTHOR.long)
            .help(constant::help_messages::AUTHOR)
            .action(ArgAction::SetTrue)
    }

    fn from_arg_matches(arg_matches: &clap::ArgMatches) -> bool {
        arg_matches.get_flag("AUTHOR")
    }
}
