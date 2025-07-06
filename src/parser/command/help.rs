use clap::{Arg, ArgAction, Command};

use super::ClapArg;
use crate::{
    constant,
    core::{ExitKind, ProgramExit},
    helper,
};

pub struct HelpClapArg;

impl HelpClapArg {
    pub fn as_program_exit(cli_parser: &Command) -> ProgramExit {
        let rendered_help = cli_parser.clone().render_help();

        ProgramExit {
            message: rendered_help.to_string().trim_end().to_string(),
            exit_status: constant::exit_status::SUCCESS,
            styled_message: Some(rendered_help.ansi().to_string().trim_end().to_string()),
            kind: ExitKind::HelpInfos,
        }
    }
}

impl ClapArg<bool> for HelpClapArg {
    fn build() -> Arg {
        Arg::new("help")
            .id("HELP")
            .short(helper::to_char(constant::cli_options::HELP.short))
            .long(constant::cli_options::HELP.long)
            .help(constant::help_messages::HELP)
            .action(ArgAction::SetTrue)
    }

    fn from_arg_matches(arg_matches: &clap::ArgMatches) -> bool {
        arg_matches.get_flag("HELP")
    }
}
