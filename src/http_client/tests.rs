use std::{collections::HashMap, time::Duration};

use mockito::Server;

use super::*;
use crate::{
    constant,
    core::{ExitKind, ProgramExit},
};

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
                let http_client = UreqHttpClient::default();

                let actual = http_client.get(&format!("{server_url}{mock_uri}"));
                let expected: Result<String, ProgramExit> = Ok(String::from(mock_body));

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
                let http_client = UreqHttpClient {
                    server_url,
                    global_timeout: None,
                };

                let actual = http_client.get(mock_uri);
                let expected: Result<String, ProgramExit> = Ok(String::from(mock_body));

                mock.assert();
                assert_eq!(actual, expected);
            }

            #[test]
            fn it_fetches_data_as_string_with_given_timeout() {
                let mut mock_server = Server::new();
                let mock_body = "gitignore template for rust";
                let mock_uri = "/api/rust";
                let mock = mock_server
                    .mock("GET", mock_uri)
                    .with_status(200)
                    .with_body(mock_body)
                    .create();

                let server_url = mock_server.url();
                let http_client = UreqHttpClient {
                    server_url,
                    global_timeout: Some(Duration::from_secs(5)),
                };

                let actual = http_client.get(mock_uri);
                let expected: Result<String, ProgramExit> = Ok(String::from(mock_body));

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
                let http_client = UreqHttpClient::default();

                let actual = http_client.get(&format!("{server_url}{mock_uri}"));
                let expected: Result<String, ProgramExit> = Err(ProgramExit {
                    message: constant::error_messages::API_CALL_FAILURE
                        .replace("{error}", constant::error_messages::HTTP_400),
                    exit_status: constant::exit_status::GENERIC,
                    styled_message: None,
                    kind: ExitKind::Error,
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
                let http_client = UreqHttpClient::default();

                let actual = http_client.get(&format!("{server_url}{mock_uri}"));
                let expected: Result<String, ProgramExit> = Err(ProgramExit {
                    message: String::from(constant::error_messages::INVALID_ENCODING),
                    exit_status: constant::exit_status::HTTP_CLIENT_ERROR,
                    styled_message: None,
                    kind: ExitKind::Error,
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
                let http_client = MockHttpClient {
                    response: Ok(String::from(result_content)),
                };

                let actual = http_client.get("/api/rust");
                let expected: Result<String, ProgramExit> = Ok(String::from(result_content));

                assert_eq!(actual, expected);
            }

            #[test]
            fn it_returns_error_mocked_response() {
                let result_content = "error response";
                let http_client = MockHttpClient {
                    response: Err(ProgramExit {
                        message: String::from(result_content),
                        exit_status: constant::exit_status::GENERIC,
                        styled_message: None,
                        kind: ExitKind::Error,
                    }),
                };

                let actual = http_client.get("/api/rust");
                let expected: Result<String, ProgramExit> = Err(ProgramExit {
                    message: String::from(result_content),
                    exit_status: constant::exit_status::GENERIC,
                    styled_message: None,
                    kind: ExitKind::Error,
                });

                assert_eq!(actual, expected);
            }
        }
    }
}

mod mock_endpoint_http_client {
    use super::*;

    mod get {
        use super::*;

        mod success {
            use super::*;

            #[test]
            fn it_returns_ok_mocked_response_for_given_url() {
                let rust_result_content = "success rust response";
                let python_result_content = "success python response";
                let http_client = MockEndpointHttpClient {
                    response: HashMap::from([
                        (
                            "/api/rust".to_string(),
                            Ok(String::from(rust_result_content)),
                        ),
                        (
                            "/api/python".to_string(),
                            Ok(String::from(python_result_content)),
                        ),
                    ]),
                };

                let actual = http_client.get("/api/rust");
                let expected: Result<String, ProgramExit> = Ok(String::from(rust_result_content));

                assert_eq!(actual, expected);
            }

            #[test]
            fn it_returns_error_mocked_response_for_given_url() {
                let rust_result_content = "success rust response";
                let python_result_content = "success python response";
                let http_client = MockEndpointHttpClient {
                    response: HashMap::from([
                        (
                            "/api/rust".to_string(),
                            Err(ProgramExit {
                                message: String::from(rust_result_content),
                                exit_status: constant::exit_status::GENERIC,
                                styled_message: None,
                                kind: ExitKind::Error,
                            }),
                        ),
                        (
                            "/api/python".to_string(),
                            Err(ProgramExit {
                                message: String::from(python_result_content),
                                exit_status: constant::exit_status::GENERIC,
                                styled_message: None,
                                kind: ExitKind::Error,
                            }),
                        ),
                    ]),
                };

                let actual = http_client.get("/api/python");
                let expected: Result<String, ProgramExit> = Err(ProgramExit {
                    message: String::from(python_result_content),
                    exit_status: constant::exit_status::GENERIC,
                    styled_message: None,
                    kind: ExitKind::Error,
                });

                assert_eq!(actual, expected);
            }
        }
    }
}
