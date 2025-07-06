use clap::{Command, Error};

pub use crate::core::impls::{
    GitignoreTemplateManager, LocalGitignoreTemplateManager, RemoteGitignoreTemplateManager,
};
use crate::{
    constant::{error_messages, exit_status, help_texts},
    parser::Args,
    printer::{Data, ppg},
};

/// DTO struct representing an early or abrupt program exit.
#[derive(Clone, PartialEq, Debug)]
pub struct ProgramExit {
    /// The message linked to the program exit.
    pub message: String,

    /// The exit status code to be returned by the script.
    pub exit_status: i32,

    /// The ANSI-styled message linked to the program exit.
    ///
    /// Same as [`ProgramExit::message`] but styled.
    pub styled_message: Option<String>,

    /// The kind of program exit.
    pub kind: ExitKind,
}

impl ProgramExit {
    pub fn success(message: &String, kind: &ExitKind) -> Self {
        Self {
            message: message.clone(),
            exit_status: exit_status::SUCCESS,
            styled_message: None,
            kind: kind.clone(),
        }
    }

    pub fn styled_success(message: &String, styled_message: &String, kind: &ExitKind) -> Self {
        Self {
            message: message.clone(),
            exit_status: exit_status::SUCCESS,
            styled_message: Some(styled_message.clone()),
            kind: kind.clone(),
        }
    }

    pub fn from_clap_error(error: &Error) -> Self {
        Self {
            message: ppg(&Data::ClapError(error)),
            exit_status: error.exit_code(),
            styled_message: Some(ppg(&Data::StyledClapError(error))),
            kind: ExitKind::Error,
        }
    }

    pub fn author_option(cli_parser: &Command) -> Self {
        let message = match cli_parser.get_author() {
            Some(author) => author,
            None => error_messages::AUTHOR_INFOS_NOT_AVAILABLE,
        };

        Self::success(&message.to_string(), &ExitKind::AuthorInfos)
    }

    pub fn version_option(cli_parser: &Command) -> Self {
        let message = match cli_parser.get_version() {
            Some(version) => format!("{} {version}", env!("CARGO_PKG_NAME")),
            None => error_messages::VERSION_INFOS_NOT_AVAILABLE.to_string(),
        };

        Self::success(&message, &ExitKind::VersionInfos)
    }

    pub fn help_option(cli_parser: &Command) -> Self {
        let rendered_help = cli_parser.clone().render_help();

        Self::styled_success(
            &rendered_help.to_string().trim_end().to_string(),
            &rendered_help.ansi().to_string().trim_end().to_string(),
            &ExitKind::HelpInfos,
        )
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct QualifiedString {
    pub value: String,
    pub kind: StringKind,
}

impl QualifiedString {
    pub fn empty(kind: StringKind) -> Self {
        Self {
            value: help_texts::NOTHING_TO_BE_PRINTED.to_string(),
            kind,
        }
    }
}

/// Enum for kind of program exit.
#[derive(Clone, PartialEq, Debug)]
pub enum ExitKind {
    /// Early program exit to print version infos.
    VersionInfos,

    /// Early program exit to print help infos.
    HelpInfos,

    /// Early program exit to print author infos.
    AuthorInfos,

    /// Abrupt program exit due to runtime error.
    Error,
}

#[derive(Clone, PartialEq, Debug, Copy)]
pub enum StringKind {
    Remote,
    Local,
    Mixed,
}

/// Template generator trait to generate string templates.
pub trait TemplateGenerator: TemplateLister {
    /// Generates a string template matching given template names.
    ///
    /// Template generation source is not taken into consideration here. It is
    /// up to the struct implementing this trait to take that decision.
    ///
    /// # Arguments
    ///
    /// * `template_names` - The template names to be used to generate the
    ///   actual template.
    ///
    /// # Returns
    ///
    /// A result containing the generated template on success, or a
    /// [`ProgramExit`] on error (e.g. 4xx, network issues...).
    fn generate(&self, template_names: &[String]) -> Result<QualifiedString, ProgramExit>;

    /// Generates a string template matching given template names, with robust
    /// template names check.
    ///
    /// Behaves the same as [`TemplateGenerator::generate`] but will
    /// return a detailed error message in case any of provided template
    /// names are not listed in available template list (as returned by
    /// [`TemplateLister::list`]).
    ///
    /// # Arguments
    ///
    /// * `template_names` - The template names to be used to generate the
    ///   actual template.
    ///
    /// # Returns
    ///
    /// A result containing the generated template on success, or a
    /// [`ProgramExit`] on error (e.g. 4xx, network issues, template not
    /// found...).
    fn generate_with_template_check(
        &self,
        template_names: &[String],
    ) -> Result<QualifiedString, ProgramExit>;
}

/// Template lister trait to list available templates.
pub trait TemplateLister {
    /// Lists available templates.
    ///
    /// Template listing source is not taken into consideration here. It is
    /// up to the struct implementing this trait to take that decision.
    ///
    /// # Returns
    ///
    /// A result containing the list of available templates on success, or a
    /// [`ProgramExit`] on error (e.g. file system failure, insufficient
    /// privilege...).
    fn list(&self) -> Result<QualifiedString, ProgramExit>;
}

pub trait TemplateFactory<T: TemplateManager + ?Sized> {
    fn from_args(args: &Args) -> Result<Box<T>, ProgramExit>;
}

pub trait TemplateManager: TemplateGenerator {}
