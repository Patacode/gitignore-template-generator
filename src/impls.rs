use std::process::exit;

use crate::api::{HttpClient, ProgramError};

pub struct UreqClient;
pub struct MockClient {
    response: Result<String, ProgramError>,
}

impl HttpClient for UreqClient {
    fn get(&self, url: &str) -> Result<String, ProgramError> {
        let result = ureq::get(url).call();

        match result {
            Ok(mut response) => match response.body_mut().read_to_string() {
                Ok(body) => Ok(body),
                Err(_error) => Err(ProgramError {
                    message: String::from(
                        "An error occurred during body parsing",
                    ),
                    exit_status: 3,
                }),
            },
            Err(error) => Err(ProgramError {
                message: format!(
                    "An error occurred during the API call: {error}"
                ),
                exit_status: 2,
            }),
        }
    }
}

impl HttpClient for MockClient {
    fn get(&self, _url: &str) -> Result<String, ProgramError> {
        self.response.clone()
    }
}

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
