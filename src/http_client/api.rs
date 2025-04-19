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
    use crate::constant;
    use mockito::Server;

    mod ureq_client {
        use super::*;

        mod get {
            use super::*;

            mod success {
                use super::*;

                #[test]
                fn it_fetches_data_as_string_when_200_response() {
                    let mut mock_server = Server::new();
                    let mock_body = "gitignore template for rust";
                    let mock_uri = "/api/rust";
                    let mock = mock_server
                        .mock("GET", mock_uri)
                        .with_status(200)
                        .with_body(mock_body)
                        .create();

                    let server_url = mock_server.url();
                    let http_client = UreqClient::default();

                    let actual =
                        http_client.get(&format!("{server_url}{mock_uri}"));
                    let expected: Result<String, ProgramError> =
                        Ok(String::from(mock_body));

                    mock.assert();
                    assert_eq!(actual, expected);
                }

                #[test]
                fn it_fetches_data_as_string_from_given_server_url_when_200() {
                    let mut mock_server = Server::new();
                    let mock_body = "gitignore template for rust";
                    let mock_uri = "/api/rust";
                    let mock = mock_server
                        .mock("GET", mock_uri)
                        .with_status(200)
                        .with_body(mock_body)
                        .create();

                    let server_url = mock_server.url();
                    let http_client = UreqClient { server_url };

                    let actual = http_client.get(mock_uri);
                    let expected: Result<String, ProgramError> =
                        Ok(String::from(mock_body));

                    mock.assert();
                    assert_eq!(actual, expected);
                }
            }

            mod failure {
                use super::*;

                #[test]
                fn it_fails_with_api_call_error_when_400_response() {
                    let mut mock_server = Server::new();
                    let mock_body = "error response";
                    let mock_uri = "/api/rust";
                    let mock = mock_server
                        .mock("GET", mock_uri)
                        .with_status(400)
                        .with_body(mock_body)
                        .create();

                    let server_url = mock_server.url();
                    let http_client = UreqClient::default();

                    let actual =
                        http_client.get(&format!("{server_url}{mock_uri}"));
                    let expected: Result<String, ProgramError> =
                        Err(ProgramError {
                            message: constant::error_messages::API_CALL_FAILURE
                                .replace(
                                    "{error}",
                                    constant::error_messages::HTTP_400,
                                ),
                            exit_status: constant::exit_status::GENERIC,
                        });

                    mock.assert();
                    assert_eq!(actual, expected);
                }

                #[test]
                fn it_fails_with_body_parsing_error_when_invalid_body() {
                    let mut mock_server = Server::new();
                    let mock_body = vec![0, 159, 146, 150];
                    let mock_uri = "/api/rust";
                    let mock = mock_server
                        .mock("GET", mock_uri)
                        .with_status(200)
                        .with_body(mock_body)
                        .create();

                    let server_url = mock_server.url();
                    let http_client = UreqClient::default();

                    let actual =
                        http_client.get(&format!("{server_url}{mock_uri}"));
                    let expected: Result<String, ProgramError> =
                        Err(ProgramError {
                            message: String::from(
                                constant::error_messages::BODY_PARSING_ISSUE,
                            ),
                            exit_status:
                                constant::exit_status::BODY_PARSING_ISSUE,
                        });

                    mock.assert();
                    assert_eq!(actual, expected);
                }
            }
        }
    }

    mod mock_client {
        use super::*;

        mod get {
            use super::*;

            mod success {
                use super::*;

                #[test]
                fn it_returns_ok_mocked_response() {
                    let result_content = "success response";
                    let http_client = MockClient {
                        response: Ok(String::from(result_content)),
                    };

                    let actual = http_client.get("/api/rust");
                    let expected: Result<String, ProgramError> =
                        Ok(String::from(result_content));

                    assert_eq!(actual, expected);
                }

                #[test]
                fn it_returns_error_mocked_response() {
                    let result_content = "error response";
                    let http_client = MockClient {
                        response: Err(ProgramError {
                            message: String::from(result_content),
                            exit_status: constant::exit_status::GENERIC,
                        }),
                    };

                    let actual = http_client.get("/api/rust");
                    let expected: Result<String, ProgramError> =
                        Err(ProgramError {
                            message: String::from(result_content),
                            exit_status: constant::exit_status::GENERIC,
                        });

                    assert_eq!(actual, expected);
                }
            }
        }
    }
}
