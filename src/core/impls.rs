use crate::{constant, http_client::HttpClient};

use super::{ProgramExit, TemplateGenerator};

/// Generator of gitignore templates.
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
        template_names: &[String],
    ) -> Result<String, ProgramExit> {
        let uri = constant::template_generator::URI;
        let path_param = template_names.join(",");
        let full_uri = format!("{uri}/{path_param}");
        http_client.get(&full_uri)
    }
}
