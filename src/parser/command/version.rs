use clap::{Arg, ArgAction, Command};

use super::ClapArg;
use crate::{
    constant,
    core::{ExitKind, ProgramExit},
    helper,
};

pub struct VersionClapArg;

impl VersionClapArg {
    pub fn as_program_exit(cli_parser: &Command) -> ProgramExit {
        let message = match cli_parser.get_version() {
            Some(version) => format!("{} {version}", env!("CARGO_PKG_NAME")),
            None => constant::error_messages::VERSION_INFOS_NOT_AVAILABLE.to_string(),
        };

        ProgramExit {
            message,
            exit_status: constant::exit_status::SUCCESS,
            styled_message: None,
            kind: ExitKind::VersionInfos,
        }
    }
}

impl ClapArg<bool> for VersionClapArg {
    fn build() -> Arg {
        Arg::new("version")
            .id("VERSION")
            .short(helper::to_char(constant::cli_options::VERSION.short))
            .long(constant::cli_options::VERSION.long)
            .help(constant::help_messages::VERSION)
            .action(ArgAction::SetTrue)
    }

    fn from_arg_matches(arg_matches: &clap::ArgMatches) -> bool {
        arg_matches.get_flag("VERSION")
    }
}
