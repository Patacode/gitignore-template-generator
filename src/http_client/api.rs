pub use crate::http_client::impls::UreqClient;

#[derive(Clone, PartialEq, Debug)]
pub struct ProgramError {
    pub message: String,
    pub exit_status: i32,
}

pub trait HttpClient {
    fn get(&self, url: &str) -> Result<String, ProgramError>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::Server;

    #[test]
    fn it_fetches_data_correctly_with_ureq_client() {
        let api_response = "gitignore template for rust";
        let mut server = Server::new();
        let base_url = server.url();
        let uri = "/api/rust";
        let mock = server.mock("GET", uri)
            .with_status(200)
            .with_body(api_response)
            .create();

        let client = UreqClient {};
        let expected: Result<String, ProgramError> = Ok(String::from(api_response));
        let actual = client.get(&format!("{base_url}{uri}"));

        mock.assert();
        assert_eq!(actual, expected);
    }
}
