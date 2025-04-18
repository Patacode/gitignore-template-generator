use crate::http_client::{HttpClient, ProgramError, UreqClient};

use super::TemplateGenerator;

pub struct GitignoreTemplateGenerator;

impl TemplateGenerator for GitignoreTemplateGenerator {
    fn generate(values: &String) -> Result<String, ProgramError> {
        let client = UreqClient {};
        let url =
            format!("https://www.toptal.com/developers/gitignore/api/{values}");

        client.get(&url)
    }
}
