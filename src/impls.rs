use std::process::exit;

use crate::http_client::{HttpClient, UreqClient};

pub fn get_call_to_gitignore_template_service(values: &String) -> String {
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
