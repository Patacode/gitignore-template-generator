pub use crate::http_client::impls::{MockClient, UreqClient};

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

        let mock = server
            .mock("GET", uri)
            .with_status(200)
            .with_body(api_response)
            .create();

        let client = UreqClient::default();
        let expected: Result<String, ProgramError> =
            Ok(String::from(api_response));
        let actual = client.get(&format!("{base_url}{uri}"));

        mock.assert();
        assert_eq!(actual, expected);
    }

    #[test]
    fn it_gracefully_fails_if_non_200_response_with_ureq_client() {
        let api_response = "error response";
        let mut server = Server::new();
        let base_url = server.url();
        let uri = "/api/rust";

        let mock = server
            .mock("GET", uri)
            .with_status(400)
            .with_body(api_response)
            .create();

        let client = UreqClient::default();
        let expected: Result<String, ProgramError> = Err(ProgramError {
            message: String::from(
                "An error occurred during the API call: http status: 400",
            ),
            exit_status: 2,
        });
        let actual = client.get(&format!("{base_url}{uri}"));

        mock.assert();
        assert_eq!(actual, expected);
    }

    #[test]
    fn it_gracefully_fails_if_body_parsing_issue_with_ureq_client() {
        let mut server = Server::new();
        let base_url = server.url();
        let uri = "/api/rust";

        let mock = server
            .mock("GET", uri)
            .with_status(200)
            .with_body(vec![0, 159, 146, 150])
            .create();

        let client = UreqClient::default();
        let expected: Result<String, ProgramError> = Err(ProgramError {
            message: String::from("An error occurred during body parsing"),
            exit_status: 3,
        });
        let actual = client.get(&format!("{base_url}{uri}"));

        mock.assert();
        assert_eq!(actual, expected);
    }

    #[test]
    fn it_returns_given_result_with_mock_client() {
        let client = MockClient {
            response: Ok(String::from("success response")),
        };

        let expected: Result<String, ProgramError> =
            Ok(String::from("success response"));
        let actual = client.get("/api/rust");

        assert_eq!(actual, expected);
    }
}
