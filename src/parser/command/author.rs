use clap::{Arg, ArgAction, ArgMatches, Command};

use super::ClapArg;
use crate::{
    constant::{self, error_messages},
    core::{ExitKind, ProgramExit},
    helper::{DefaultUtils, Utils},
};

pub struct AuthorClapArg;

impl AuthorClapArg {
    pub fn as_program_exit(cli_parser: &Command) -> ProgramExit {
        let message = match cli_parser.get_author() {
            Some(author) => author,
            None => error_messages::AUTHOR_INFOS_NOT_AVAILABLE,
        };

        ProgramExit::success(message, &ExitKind::AuthorInfos)
    }
}

impl ClapArg<bool> for AuthorClapArg {
    fn build() -> Arg {
        Arg::new("author")
            .id("AUTHOR")
            .short(DefaultUtils::to_char(constant::cli_options::AUTHOR.short))
            .long(constant::cli_options::AUTHOR.long)
            .help(constant::help_messages::AUTHOR)
            .action(ArgAction::SetTrue)
    }

    fn from_arg_matches(arg_matches: &ArgMatches) -> bool {
        arg_matches.get_flag("AUTHOR")
    }
}
