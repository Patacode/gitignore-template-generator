use clap::{Arg, ArgAction, ArgMatches, Command};

use super::ClapArg;
use crate::{
    constant::{self, error_messages},
    core::{ExitKind, ProgramExit},
    helper::{DefaultUtils, Utils},
};

pub struct VersionClapArg;

impl VersionClapArg {
    pub fn as_program_exit(cli_parser: &Command) -> ProgramExit {
        let message = match cli_parser.get_version() {
            Some(version) => format!("{} {version}", env!("CARGO_PKG_NAME")),
            None => error_messages::VERSION_INFOS_NOT_AVAILABLE.to_string(),
        };

        ProgramExit::success(&message, &ExitKind::VersionInfos)
    }
}

impl ClapArg<bool> for VersionClapArg {
    fn build() -> Arg {
        Arg::new("version")
            .id("VERSION")
            .short(DefaultUtils::to_char(constant::cli_options::VERSION.short))
            .long(constant::cli_options::VERSION.long)
            .help(constant::help_messages::VERSION)
            .action(ArgAction::SetTrue)
    }

    fn from_arg_matches(arg_matches: &ArgMatches) -> bool {
        arg_matches.get_flag("VERSION")
    }
}
