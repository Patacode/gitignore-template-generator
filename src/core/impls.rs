use super::{ProgramExit, TemplateGenerator, TemplateLister};
use crate::http_client::HttpClient;

/// Manager of gitignore templates.
/// 
/// It can generate and list gitignore templates.
pub struct GitignoreTemplateGenerator;

impl TemplateGenerator for GitignoreTemplateGenerator {
    /// Generates a gitignore template matching given template names.
    ///
    /// * Each associated gitignore template will be merged into one big string
    /// * A 404 error will be returned if any of the given template names isn't
    ///     supported by the underlying template generator API.
    ///
    /// See [`TemplateGenerator::generate_from_api`] for more infos.
    fn generate_from_api(
        http_client: &impl HttpClient,
        endpoint_uri: &str,
        template_names: &[String],
    ) -> Result<String, ProgramExit> {
        let path_param = template_names.join(",");
        let full_uri = format!("{endpoint_uri}/{path_param}");
        http_client.get(&full_uri)
    }
}

impl TemplateLister for GitignoreTemplateGenerator {
    fn list_from_api(
        http_client: &impl HttpClient,
        endpoint_uri: &str,
    ) -> Result<String, ProgramExit> {
        http_client.get(endpoint_uri)
    }
}
