use crate::{constant, http_client::{HttpClient, ProgramError}};

use super::TemplateGenerator;

pub struct GitignoreTemplateGenerator;

impl TemplateGenerator for GitignoreTemplateGenerator {
    fn generate_from_api(
        http_client: &dyn HttpClient,
        values: &str,
    ) -> Result<String, ProgramError> {
        let uri = constant::template_generator::URI;
        let full_uri = format!("{uri}/{values}");
        http_client.get(&full_uri)
    }
}
