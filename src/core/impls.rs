use super::{ProgramExit, TemplateGenerator, TemplateLister};
use crate::{
    constant::template_manager::{GENERATOR_URI, LISTER_URI},
    http_client::HttpClient,
};

/// Manager of gitignore templates.
///
/// It can generate and list gitignore templates.
pub struct GitignoreTemplateManager;

impl GitignoreTemplateManager {
    fn parse_template_list_from_api(template_list: String) -> String {
        template_list.replace(',', "\n")
    }
}

impl TemplateGenerator for GitignoreTemplateManager {
    /// Generates a gitignore template matching given template names.
    ///
    /// * Each associated gitignore template will be merged into one big string
    /// * A 404 error will be returned if any of the given template names isn't
    ///     supported by the underlying template generator API.
    ///
    /// See [`TemplateGenerator::generate_from_api`] for more infos.
    fn generate_from_api(
        http_client: &impl HttpClient,
        endpoint_uri: Option<&str>,
        template_names: &[String],
    ) -> Result<String, ProgramExit> {
        let path_param = template_names.join(",");
        let endpoint_uri = endpoint_uri.unwrap_or(GENERATOR_URI);
        let full_uri = format!("{endpoint_uri}/{path_param}");

        http_client.get(&full_uri)
    }
}

impl TemplateLister for GitignoreTemplateManager {
    fn list_from_api(
        http_client: &impl HttpClient,
        endpoint_uri: Option<&str>,
    ) -> Result<String, ProgramExit> {
        let endpoint_uri = endpoint_uri.unwrap_or(LISTER_URI);
        match http_client.get(endpoint_uri) {
            Ok(result) => Ok(Self::parse_template_list_from_api(result)),
            Err(error) => Err(error),
        }
    }
}
