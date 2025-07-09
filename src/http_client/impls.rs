use std::time::Duration;

use ureq::Agent;

use crate::{
    constant::{error_messages, exit_status, template_manager},
    core::{ExitKind, ProgramExit},
    http_client::{HttpClient, MockEndpointHttpClient, MockHttpClient, UreqHttpClient},
};

impl HttpClient for UreqHttpClient {
    /// Make a GET HTTP call using a [`ureq`] client.
    ///
    /// The server base url (i.e. https://localhost:8080) should be provided
    /// as part of [`UreqHttpClient::server_url] field.
    ///
    /// See [`HttpClient::get`] for more infos.
    fn get(&self, url: &str) -> Result<String, ProgramExit> {
        let full_url = format!("{}{url}", self.server_url);
        let agent: Agent = Agent::config_builder()
            .timeout_global(Some(
                self.global_timeout.unwrap_or(Duration::from_secs(
                    template_manager::TIMEOUT
                        .parse()
                        .expect(error_messages::FAILED_U64_CONVERSION),
                )),
            ))
            .build()
            .into();

        let result = agent.get(full_url).call();

        match result {
            Ok(mut response) => match response.body_mut().read_to_string() {
                Ok(body) => Ok(body.trim().to_string()),
                Err(error) => Err(ProgramExit {
                    message: error.to_string(),
                    exit_status: exit_status::HTTP_CLIENT_ERROR,
                    styled_message: None,
                    kind: ExitKind::Error,
                }),
            },
            Err(error) => Err(ProgramExit {
                message: error_messages::API_CALL_FAILURE.replace("{error}", &error.to_string()),
                exit_status: exit_status::GENERIC,
                styled_message: None,
                kind: ExitKind::Error,
            }),
        }
    }
}

impl HttpClient for MockHttpClient {
    /// Returns the result linked to this instance.
    ///
    /// The given `_url` will not be used, simply a clone of linked
    /// result will be returned.
    fn get(&self, _url: &str) -> Result<String, ProgramExit> {
        self.response.clone()
    }
}

impl HttpClient for MockEndpointHttpClient {
    /// Returns the result linked to the given url.
    ///
    /// The given `url` will only be used to get proper result from linked
    /// hashmap.
    fn get(&self, url: &str) -> Result<String, ProgramExit> {
        println!("{:?}", self.response[url]);

        match self.response.get(url) {
            Some(value) => value.clone(),
            None => Err(ProgramExit::error(
                &error_messages::INVALID_MAPPED_URI.replace("{uri}", url),
            )),
        }
    }
}
