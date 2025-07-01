use crate::core::ProgramExit;
pub use crate::http_client::impls::{MockEndpointHttpClient, MockHttpClient, UreqHttpClient};

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
