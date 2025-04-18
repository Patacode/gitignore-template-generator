use crate::http_client::api::{HttpClient, ProgramError};

pub struct UreqClient {
    pub server_url: String,
}
pub struct MockClient {
    pub response: Result<String, ProgramError>,
}

impl UreqClient {
    pub fn default() -> Self {
        Self {
            server_url: String::from(""),
        }
    }
}

impl HttpClient for UreqClient {
    fn get(&self, url: &str) -> Result<String, ProgramError> {
        let full_url = format!("{}{url}", self.server_url);
        let result = ureq::get(full_url).call();

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
