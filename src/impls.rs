use std::process::exit;

use crate::api::HttpClient;

pub struct UreqClient;
pub struct MockClient {
    response: Result<String, String>,
}

impl HttpClient for UreqClient {
    fn get(&self, url: &str) -> Result<String, String> {
        let result = ureq::get(url).call();

        match result {
            Ok(mut response) => match response.body_mut().read_to_string() {
                Ok(body) => Ok(body),
                Err(_error) => {
                    Err(String::from("An error occurred during body parsing"))
                }
            },
            Err(error) => {
                Err(format!("An error occurred during the API call: {error}"))
            }
        }
    }
}

impl HttpClient for MockClient {
    fn get(&self, _url: &str) -> Result<String, String> {
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
            eprintln!("{}", error);
            exit(2);
        }
    }
}
