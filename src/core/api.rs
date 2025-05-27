pub use crate::core::impls::GitignoreTemplateManager;
use crate::http_client::HttpClient;

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

/// Template generator trait to generate a template via an API call made
/// over HTTP.
pub trait TemplateGenerator: TemplateLister {
    /// Generates a string template matching given template names using local
    /// file system.
    ///
    /// It will first try to locate template dir using
    /// `GITIGNORE_TEMPLATE_GENERATOR_HOME` env var, and if not set, will then
    /// use provided `default_template_dir`.
    ///
    /// # Arguments
    ///
    /// * `default_template_dir` - The fallback directory in which templates
    ///     are stored. Will be used in case `GITIGNORE_TEMPLATE_GENERATOR_HOME`
    ///     env var is not set.
    /// * `template_names` - The template names to be used to generate the
    ///     actual template.
    ///
    /// # Returns
    ///
    /// A result containing the generated template on success, or a
    /// [`ProgramExit`] on error (e.g. file system failure, insufficient
    /// privilege...).
    fn generate_locally(
        default_template_dir: &str,
        template_names: &[String],
    ) -> Result<String, ProgramExit>;

    /// Generates a string template matching given template names through an
    /// API call.
    ///
    /// The template is generated via a GET API call made over HTTP using
    /// the given http client.
    ///
    /// # Arguments
    ///
    /// * `http_client` - The http client to be used to make the API call.
    /// * `endpoint_uri` - The endpoint URI to generate templates (defaults to
    ///     [`crate::constant::template_manager::GENERATOR_URI`] if None).
    /// * `template_names` - The template names to be used to generate the
    ///     actual template.
    ///
    /// # Returns
    ///
    /// A result containing the generated template on success, or a
    /// [`ProgramExit`] on error (e.g. 4xx, network issues...).
    fn generate_from_api(
        http_client: &impl HttpClient,
        endpoint_uri: Option<&str>,
        template_names: &[String],
    ) -> Result<String, ProgramExit>;

    /// Generates a string template matching given template names through an
    /// API call, with robust template name checks.
    ///
    /// Behaves the same as [`TemplateGenerator::generate_from_api`] but will
    /// return a detailed error message in case any of provided template
    /// names are not listed in available template list (as returned by
    /// [`TemplateLister::list_from_api`]).
    ///
    /// # Arguments
    ///
    /// * `http_client` - The http client to be used to make the API call.
    /// * `generator_endpoint_uri` - The endpoint URI to generate templates
    ///     (defaults to [`crate::constant::template_manager::GENERATOR_URI`]
    ///     if None).
    /// * `endpoint_uri` - The endpoint URI to list templates (defaults to
    ///     [`crate::constant::template_manager::LISTER_URI`] if None).
    /// * `template_names` - The template names to be used to generate the
    ///     actual template.
    ///
    /// # Returns
    ///
    /// A result containing the generated template on success, or a
    /// [`ProgramExit`] on error (e.g. 4xx, network issues, template not
    /// found...).
    fn generate_from_api_with_template_check(
        http_client: &impl HttpClient,
        generator_endpoint_uri: Option<&str>,
        lister_endpoint_uri: Option<&str>,
        template_names: &[String],
    ) -> Result<String, ProgramExit>;
}

/// Template lister trait to list available templates via an API call made
/// over HTTP.
pub trait TemplateLister {
    /// Lists available templates using local file system.
    ///
    /// It will first try to locate template dir using
    /// `GITIGNORE_TEMPLATE_GENERATOR_HOME` env var, and if not set, will then
    /// use provided `default_template_dir`.
    ///
    /// # Arguments
    ///
    /// * `default_template_dir` - The fallback directory in which templates
    ///     are stored. Will be used in case `GITIGNORE_TEMPLATE_GENERATOR_HOME`
    ///     env var is not set.
    /// * `template_names` - The template names to be used to generate the
    ///     actual template.
    ///
    /// # Returns
    ///
    /// A result containing the list of available templates on success, or a
    /// [`ProgramExit`] on error (e.g. file system failure, insufficient
    /// privilege...).
    fn list_locally(default_template_dir: &str) -> Result<String, ProgramExit>;

    /// Lists available templates through an API call.
    ///
    /// The template list is fetched via a GET API call made over HTTP using
    /// the given http client.
    ///
    /// # Arguments
    ///
    /// * `http_client` - The http client to be used to make the API call.
    /// * `endpoint_uri` - The endpoint URI to list templates (defaults to
    ///     [`crate::constant::template_manager::LISTER_URI`] if None).
    ///
    /// # Returns
    ///
    /// A result containing the template list on success, or a
    /// [`ProgramExit`] on error (e.g. 4xx, network issues...).
    fn list_from_api(
        http_client: &impl HttpClient,
        endpoint_uri: Option<&str>,
    ) -> Result<String, ProgramExit>;
}
