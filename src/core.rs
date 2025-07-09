//! Define core components used to manage gitignore templates.
use crate::{http_client::HttpClient, parser::Args};

mod impls;

#[cfg(test)]
mod tests;

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

#[derive(Clone, PartialEq, Debug)]
pub struct QualifiedString {
    pub value: String,
    pub kind: StringKind,
}

/// Manager of gitignore templates.
///
/// It can generate and list gitignore templates.
pub struct GitignoreTemplateManager {
    template_managers: Vec<Box<dyn TemplateManager>>,
}

/// Manager of gitignore templates using local filesystem.
///
/// It uses a directory on local fs as *search database*.
///
/// Whatever the action performed through it, it will always first try to
/// locate template dir using `GITIGNORE_TEMPLATE_GENERATOR_HOME` env var, and
/// if not set, will then use provided `default_template_dir`.
pub struct LocalGitignoreTemplateManager {
    /// The fallback directory in which templates
    /// are stored. Will be used in case `GITIGNORE_TEMPLATE_GENERATOR_HOME`
    /// env var is not set.
    default_template_dir: String,
}

/// Manager of gitignore templates using remote API.
///
/// The templates are managed via HTTP calls using the given `http_client`.
pub struct RemoteGitignoreTemplateManager {
    /// The http client to be used to make the API call.
    http_client: Box<dyn HttpClient>,

    /// The endpoint URI to generate templates
    /// (defaults to [`crate::constant::template_manager::GENERATOR_URI`]
    /// if None).
    generator_endpoint_uri: String,

    /// The endpoint URI to list templates (defaults to
    /// [`crate::constant::template_manager::LISTER_URI`] if None).
    lister_endpoint_uri: String,
}
