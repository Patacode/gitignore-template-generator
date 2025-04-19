use crate::{
    constant,
    http_client::api::{HttpClient, ProgramError},
};

#[derive(Default)]
pub struct UreqClient {
    pub server_url: String,
}
pub struct MockClient {
    pub response: Result<String, ProgramError>,
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
                        constant::error_messages::BODY_PARSING_ISSUE,
                    ),
                    exit_status: constant::exit_status::BODY_PARSING_ISSUE,
                }),
            },
            Err(error) => Err(ProgramError {
                message: constant::error_messages::API_CALL_FAILURE
                    .replace("{error}", &error.to_string()),
                exit_status: constant::exit_status::GENERIC,
            }),
        }
    }
}

impl HttpClient for MockClient {
    fn get(&self, _url: &str) -> Result<String, ProgramError> {
        self.response.clone()
    }
}
