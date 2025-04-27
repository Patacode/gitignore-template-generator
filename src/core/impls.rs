use crate::{constant, http_client::HttpClient};

use super::{ProgramError, TemplateGenerator};

pub struct GitignoreTemplateGenerator;

impl TemplateGenerator for GitignoreTemplateGenerator {
    fn generate_from_api(
        http_client: &dyn HttpClient,
        template_names: &[String],
    ) -> Result<String, ProgramError> {
        let uri = constant::template_generator::URI;
        let path_param = template_names.join(",");
        let full_uri = format!("{uri}/{path_param}");
        http_client.get(&full_uri)
    }
}
