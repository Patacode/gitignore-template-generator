use clap::{Arg, ArgAction, Command};

use super::ClapArg;
use crate::{
    constant,
    core::{ExitKind, ProgramExit},
    helper,
};

pub struct AuthorClapArg;

impl AuthorClapArg {
    pub fn as_program_exit(cli_parser: &Command) -> ProgramExit {
        let message = match cli_parser.get_author() {
            Some(author) => author,
            None => constant::error_messages::AUTHOR_INFOS_NOT_AVAILABLE,
        }
        .to_string();

        ProgramExit {
            message,
            exit_status: constant::exit_status::SUCCESS,
            styled_message: None,
            kind: ExitKind::AuthorInfos,
        }
    }
}

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
