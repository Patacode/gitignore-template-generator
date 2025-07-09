use std::{ffi::OsString, process::exit};

use clap::{ArgMatches, Command};

use super::{Args, ArgsParser, command::build_clap_args};
use crate::{
    constant::{parser_infos, template_manager},
    core::{ExitKind, ProgramExit},
    helper::TimeoutUnit,
    parser::{
        Action,
        command::{
            AuthorClapArg, CheckClapArg, ClapArg, GeneratorUriClapArg, HelpClapArg, ListClapArg,
            ListerUriClapArg, ServerUrlClapArg, TemplateNamesClapArg, TimeoutClapArg,
            TimeoutUnitClapArg, VersionClapArg,
        },
    },
};

type ToProgramExitCallback = fn(&Command) -> ProgramExit;

/// Default implementation of args parser that parses CLI args using
/// [`clap`].
pub struct ClapArgsParser {
    cli_parser: Command,
}

impl Default for Args {
    fn default() -> Self {
        Self::new()
    }
}

impl Args {
    pub fn new() -> Self {
        Self {
            template_names: Vec::new(),
            server_url: template_manager::BASE_URL.to_string(),
            generator_uri: template_manager::GENERATOR_URI.to_string(),
            lister_uri: template_manager::LISTER_URI.to_string(),
            show_help: false,
            show_version: false,
            show_author: false,
            show_list: false,
            check_template_names: false,
            timeout: template_manager::TIMEOUT_INT,
            timeout_unit: template_manager::TIMEOUT_UNIT_ENUM,
        }
    }

    pub fn from_arg_matches(arg_matches: &ArgMatches) -> Self {
        Self {
            template_names: TemplateNamesClapArg::from_arg_matches(arg_matches),
            server_url: ServerUrlClapArg::from_arg_matches(arg_matches),
            generator_uri: GeneratorUriClapArg::from_arg_matches(arg_matches),
            lister_uri: ListerUriClapArg::from_arg_matches(arg_matches),
            timeout: TimeoutClapArg::from_arg_matches(arg_matches),
            timeout_unit: TimeoutUnitClapArg::from_arg_matches(arg_matches),
            check_template_names: CheckClapArg::from_arg_matches(arg_matches),
            show_help: HelpClapArg::from_arg_matches(arg_matches),
            show_version: VersionClapArg::from_arg_matches(arg_matches),
            show_author: AuthorClapArg::from_arg_matches(arg_matches),
            show_list: ListClapArg::from_arg_matches(arg_matches),
        }
    }

    pub fn get_global_options(&self) -> [(bool, ToProgramExitCallback); 3] {
        [
            (self.show_help, HelpClapArg::as_program_exit),
            (self.show_version, VersionClapArg::as_program_exit),
            (self.show_author, AuthorClapArg::as_program_exit),
        ]
    }

    pub fn get_action_options(&self) -> [(bool, Action); 2] {
        [
            (self.show_list, Action::List),
            (self.check_template_names, Action::RobustGenerate),
        ]
    }

    pub fn to_action(&self) -> Action {
        self.get_action_options()
            .into_iter()
            .find_map(|(flag, action)| flag.then_some(action))
            .unwrap_or(Action::Generate)
    }

    pub fn as_program_exit(&self, cli_parser: &Command) -> Option<ProgramExit> {
        self.get_global_options()
            .into_iter()
            .find_map(|(flag, handler)| flag.then(|| handler(cli_parser)))
    }

    /// Sets new value for `template_names` field.
    ///
    /// It needs to be called on struct instance and effectively mutates it.
    ///
    /// # Arguments
    ///
    /// * `template_names` - The new value to be assigned to `template_names`
    ///   field.
    ///
    /// # Returns
    ///
    /// The mutated borrowed instance.
    pub fn with_template_names(mut self, template_names: Vec<String>) -> Self {
        self.template_names = template_names;
        self
    }

    /// Sets new value for `server_url` field.
    ///
    /// It needs to be called on struct instance and effectively mutates it.
    ///
    /// # Arguments
    ///
    /// * `server_url` - The new value to be assigned to `server_url`
    ///   field.
    ///
    /// # Returns
    ///
    /// The mutated borrowed instance.
    pub fn with_server_url(mut self, server_url: &str) -> Self {
        self.server_url = server_url.to_string();
        self
    }

    /// Sets new value for `generator_uri` field.
    ///
    /// It needs to be called on struct instance and effectively mutates it.
    ///
    /// # Arguments
    ///
    /// * `generator_uri` - The new value to be assigned to
    ///   `generator_uri` field.
    ///
    /// # Returns
    ///
    /// The mutated borrowed instance.
    pub fn with_generator_uri(mut self, generator_uri: &str) -> Self {
        self.generator_uri = generator_uri.to_string();
        self
    }

    /// Sets new value for `lister_uri` field.
    ///
    /// It needs to be called on struct instance and effectively mutates it.
    ///
    /// # Arguments
    ///
    /// * `lister_uri` - The new value to be assigned to `lister_uri` field.
    ///
    /// # Returns
    ///
    /// The mutated borrowed instance.
    pub fn with_lister_uri(mut self, lister_uri: &str) -> Self {
        self.lister_uri = lister_uri.to_string();
        self
    }

    /// Sets new value for `show_list` field.
    ///
    /// It needs to be called on struct instance and effectively mutates it.
    ///
    /// # Arguments
    ///
    /// * `show_list` - The new value to be assigned to `show_list`
    ///   field.
    ///
    /// # Returns
    ///
    /// The mutated borrowed instance.
    pub fn with_show_list(mut self, show_list: bool) -> Self {
        self.show_list = show_list;
        self
    }

    /// Sets new value for `check_template_names` field.
    ///
    /// It needs to be called on struct instance and effectively mutates it.
    ///
    /// # Arguments
    ///
    /// * `check_template_names` - The new value to be assigned to
    ///   `check_template_names` field.
    ///
    /// # Returns
    ///
    /// The mutated borrowed instance.
    pub fn with_check_template_names(mut self, check_template_names: bool) -> Self {
        self.check_template_names = check_template_names;
        self
    }

    /// Sets new value for `timeout` field.
    ///
    /// It needs to be called on struct instance and effectively mutates it.
    ///
    /// # Arguments
    ///
    /// * `timeout` - The new value to be assigned to `timeout` field.
    ///
    /// # Returns
    ///
    /// The mutated borrowed instance.
    pub fn with_timeout(mut self, timeout: u64) -> Self {
        self.timeout = timeout;
        self
    }

    /// Sets new value for `timeout_unit` field.
    ///
    /// It needs to be called on struct instance and effectively mutates it.
    ///
    /// # Arguments
    ///
    /// * `timeout_unit` - The new value to be assigned to `timeout_unit` field.
    ///
    /// # Returns
    ///
    /// The mutated borrowed instance.
    pub fn with_timeout_unit(mut self, timeout_unit: TimeoutUnit) -> Self {
        self.timeout_unit = timeout_unit;
        self
    }
}

impl Default for ClapArgsParser {
    fn default() -> Self {
        Self::new()
    }
}

impl ClapArgsParser {
    pub fn new() -> Self {
        Self {
            cli_parser: Command::new(env!("CARGO_PKG_NAME"))
                .version(env!("CARGO_PKG_VERSION"))
                .author(env!("CARGO_PKG_AUTHORS"))
                .about(parser_infos::ABOUT)
                .help_template(include_str!("../../assets/help_template.txt"))
                .disable_help_flag(true)
                .disable_version_flag(true)
                .args(build_clap_args()),
        }
    }

    fn print_message(error: &ProgramExit, message: &str) -> Option<String> {
        match error.kind {
            ExitKind::Error => eprintln!("{message}"),
            _ => println!("{message}"),
        }
        Some(message.to_string())
    }

    fn process_arg_matches(&self, arg_matches: &ArgMatches) -> Result<Args, ProgramExit> {
        let args = Args::from_arg_matches(arg_matches);
        match args.as_program_exit(&self.cli_parser) {
            Some(value) => Err(value),
            None => Ok(args),
        }
    }
}

impl ArgsParser for ClapArgsParser {
    /// Parses given cli args and perform basic error handling.
    ///
    /// * If the underlying [`ProgramExit`] contains a
    ///   [`ProgramExit::styled_message`], it will be printed instead of
    ///   [`ProgramExit::message`].
    /// * Will exit using [`ProgramExit::exit_status`] if any
    ///   [`ProgramExit`] received.
    /// * Will print to stderr on error, to stdout on early exit (i.e. version,
    ///   author, help options)
    ///
    /// See [`ArgsParser::parse`] for more infos.
    fn parse(&self, args: impl IntoIterator<Item = OsString>) -> Args {
        match self.try_parse(args) {
            Ok(parsed_args) => parsed_args,
            Err(error) => {
                error
                    .styled_message
                    .clone()
                    .and_then(|styled_message| Self::print_message(&error, &styled_message))
                    .or_else(|| Self::print_message(&error, &error.message));
                exit(error.exit_status);
            }
        }
    }

    fn try_parse(&self, args: impl IntoIterator<Item = OsString>) -> Result<Args, ProgramExit> {
        match self.cli_parser.clone().try_get_matches_from(args) {
            Ok(arg_matches) => self.process_arg_matches(&arg_matches),
            Err(error) => Err(ProgramExit::from_clap_error(&error)),
        }
    }
}
