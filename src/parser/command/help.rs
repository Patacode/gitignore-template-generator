use clap::{Arg, ArgAction, ArgMatches, Command};

use super::ClapArg;
use crate::{
    constant,
    core::{ExitKind, ProgramExit},
    helper::{DefaultUtils, Utils},
};

pub struct HelpClapArg;

impl HelpClapArg {
    pub fn as_program_exit(cli_parser: &Command) -> ProgramExit {
        let rendered_help = cli_parser.clone().render_help();

        ProgramExit::styled_success(
            rendered_help.to_string().trim_end(),
            rendered_help.ansi().to_string().trim_end(),
            &ExitKind::HelpInfos,
        )
    }
}

impl ClapArg<bool> for HelpClapArg {
    fn build() -> Arg {
        Arg::new("help")
            .id("HELP")
            .short(DefaultUtils::to_char(constant::cli_options::HELP.short))
            .long(constant::cli_options::HELP.long)
            .help(constant::help_messages::HELP)
            .action(ArgAction::SetTrue)
    }

    fn from_arg_matches(arg_matches: &ArgMatches) -> bool {
        arg_matches.get_flag("HELP")
    }
}
