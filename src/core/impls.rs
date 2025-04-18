use std::process::exit;

use crate::http_client::{HttpClient, UreqClient};

use super::TemplateGenerator;

pub struct GitignoreTemplateGenerator;

impl TemplateGenerator for GitignoreTemplateGenerator {
    fn generate(values: &String) -> String {
        let client = UreqClient {};
        let url =
            format!("https://www.toptal.com/developers/gitignore/api/{values}");

        match client.get(&url) {
            Ok(result) => result,
            Err(error) => {
                eprintln!("{}", error.message);
                exit(error.exit_status);
            }
        }
    }
}
