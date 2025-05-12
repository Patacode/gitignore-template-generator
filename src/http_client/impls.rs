use std::{collections::HashMap, time::Duration};

use ureq::Agent;

use crate::{ExitKind, ProgramExit, constant, http_client::api::HttpClient};

/// Http client implementation relying on [`ureq`].
#[derive(Default)]
pub struct UreqHttpClient {
    /// The base url of the HTTP server to reach.
    ///
    /// Used as base url when calling [`UreqHttpClient::get`] method.
    pub server_url: String,

    /// The timeout for the entire HTTP call.
    pub global_timeout: Option<Duration>,
}

/// Http client implementation to mock a response.
pub struct MockHttpClient {
    /// The mocked response to be returned when calling [`MockHttpClient::get`]
    /// method.
    pub response: Result<String, ProgramExit>,
}

/// Http client implementation to mock a response.
pub struct MockEndpointHttpClient<'a> {
    /// The mocked response to be returned when calling [`MockHttpClient::get`]
    /// method.
    pub response: HashMap<&'a str, Result<String, ProgramExit>>,
}

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
                self.global_timeout
                    .unwrap_or(constant::template_manager::TIMEOUT),
            ))
            .build()
            .into();

        let result = agent.get(full_url).call();

        match result {
            Ok(mut response) => match response.body_mut().read_to_string() {
                Ok(body) => Ok(body.trim().to_string()),
                Err(_error) => Err(ProgramExit {
                    message: String::from(
                        constant::error_messages::BODY_PARSING_ISSUE,
                    ),
                    exit_status: constant::exit_status::BODY_PARSING_ISSUE,
                    styled_message: None,
                    kind: ExitKind::Error,
                }),
            },
            Err(error) => Err(ProgramExit {
                message: constant::error_messages::API_CALL_FAILURE
                    .replace("{error}", &error.to_string()),
                exit_status: constant::exit_status::GENERIC,
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

impl HttpClient for MockEndpointHttpClient<'_> {
    /// Returns the result linked to the given url.
    ///
    /// The given `url` will only be used to get proper result from linked
    /// hashmap.
    fn get(&self, url: &str) -> Result<String, ProgramExit> {
        self.response[url].clone()
    }
}
