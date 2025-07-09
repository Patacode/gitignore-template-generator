//! Define components to make HTTP calls.
use std::{collections::HashMap, time::Duration};

use crate::core::ProgramExit;

mod impls;

#[cfg(test)]
mod tests;

/// Http client trait to make HTTP calls.
pub trait HttpClient {
    /// Make a GET HTTP call to given url.
    ///
    /// # Arguments
    /// * `url` - the url to which to make the HTTP call.
    ///
    /// # Returns
    ///
    /// A result containing the response body of HTTP call if successful, or
    /// a [`ProgramExit`] on error (e.g. 4xx, network issues...).
    fn get(&self, url: &str) -> Result<String, ProgramExit>;
}

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
pub struct MockEndpointHttpClient {
    /// The mocked response to be returned when calling [`MockHttpClient::get`]
    /// method.
    pub response: HashMap<String, Result<String, ProgramExit>>,
}
