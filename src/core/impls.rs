use crate::http_client::{HttpClient, ProgramError};

use super::TemplateGenerator;

pub struct GitignoreTemplateGenerator;

impl TemplateGenerator for GitignoreTemplateGenerator {
    fn generate_from_api(
        http_client: &dyn HttpClient,
        url: &str,
        values: &String,
    ) -> Result<String, ProgramError> {
        let full_url = format!("{url}/{values}");
        http_client.get(&full_url)
    }
}
