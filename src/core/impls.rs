use crate::http_client::{HttpClient, ProgramError};

use super::TemplateGenerator;

pub struct GitignoreTemplateGenerator;

impl TemplateGenerator for GitignoreTemplateGenerator {
    fn generate_from_api(
        http_client: &dyn HttpClient,
        values: &str,
    ) -> Result<String, ProgramError> {
        let url = "https://www.toptal.com/developers/gitignore/api";
        let full_url = format!("{url}/{values}");
        http_client.get(&full_url)
    }
}
