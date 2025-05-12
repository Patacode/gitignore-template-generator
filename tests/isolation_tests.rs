use gitignore_template_generator::constant::{
    error_messages, exit_status, template_manager,
};
use mockito::Server;
use test_bin::get_test_bin;

mod success {
    use super::*;

    mod pos_args {
        use gitignore_template_generator::{
            constant, helper::load_expectation_file_as_string,
        };

        use super::*;

        #[test]
        fn it_outputs_template_when_successful_custom_generator() {
            let mut cli_tool = get_test_bin(env!("CARGO_PKG_NAME"));

            let mut mock_server = Server::new();
            let mock_server_base_url = mock_server.url();
            let template_generator_service_uri = "/custom/rust";
            let template_generator_mock = mock_server
                .mock("GET", template_generator_service_uri)
                .with_status(200)
                .with_body(load_expectation_file_as_string("rust_template"))
                .create();

            cli_tool
                .arg("rust")
                .args(["--server-url", &mock_server_base_url])
                .args(["--generator-uri", "/custom"]);
            let result = cli_tool
                .output()
                .expect(error_messages::CMD_EXECUTION_FAILURE);

            let actual_output = String::from_utf8_lossy(&result.stdout);
            let expected_output =
                load_expectation_file_as_string("rust_template");

            let actual_status_code = result.status.code();
            let expected_status_code = Some(exit_status::SUCCESS);

            template_generator_mock.assert();

            assert_eq!(actual_status_code, expected_status_code);
            assert_eq!(actual_output, expected_output);
        }

        #[test]
        fn it_outputs_template_list_when_successful_custom_lister() {
            let mut cli_tool = get_test_bin(env!("CARGO_PKG_NAME"));

            let mut mock_server = Server::new();
            let mock_server_base_url = mock_server.url();
            let template_lister_service_uri = "/custom/list";
            let template_lister_mock = mock_server
                .mock("GET", template_lister_service_uri)
                .with_status(200)
                .with_body(load_expectation_file_as_string("template_list"))
                .create();

            cli_tool
                .arg("--list")
                .args(["--server-url", &mock_server_base_url])
                .args(["--lister-uri", "/custom/list"]);
            let result = cli_tool
                .output()
                .expect(error_messages::CMD_EXECUTION_FAILURE);

            let actual_output = String::from_utf8_lossy(&result.stdout);
            let expected_output =
                load_expectation_file_as_string("template_list");

            let actual_status_code = result.status.code();
            let expected_status_code = Some(exit_status::SUCCESS);

            template_lister_mock.assert();

            assert_eq!(actual_status_code, expected_status_code);
            assert_eq!(actual_output, expected_output);
        }

        #[test]
        fn it_outputs_available_template_list() {
            let mut cli_tool = get_test_bin(env!("CARGO_PKG_NAME"));

            let mut mock_server = Server::new();
            let mock_server_base_url = mock_server.url();
            let template_generator_mock = mock_server
                .mock("GET", constant::template_manager::LISTER_URI)
                .with_status(200)
                .with_body(load_expectation_file_as_string("template_list"))
                .create();

            cli_tool
                .arg("--list")
                .args(["--server-url", &mock_server_base_url]);
            let result = cli_tool
                .output()
                .expect(error_messages::CMD_EXECUTION_FAILURE);

            let actual_output = String::from_utf8_lossy(&result.stdout);
            let expected_output =
                load_expectation_file_as_string("template_list");

            let actual_status_code = result.status.code();
            let expected_status_code = Some(exit_status::SUCCESS);

            template_generator_mock.assert();

            assert_eq!(actual_status_code, expected_status_code);
            assert_eq!(actual_output, expected_output);
        }
    }
}

mod failure {
    use super::*;

    mod pos_args {
        use gitignore_template_generator::constant;

        use super::*;

        #[test]
        fn it_outputs_error_and_fails_when_body_parsing_issue_with_generator() {
            let mut cli_tool = get_test_bin(env!("CARGO_PKG_NAME"));

            let mut mock_server = Server::new();
            let mock_server_base_url = mock_server.url();
            let template_generator_service_uri =
                format!("{}/rust", template_manager::GENERATOR_URI);
            let template_generator_mock = mock_server
                .mock("GET", template_generator_service_uri.as_str())
                .with_status(200)
                .with_body(vec![0, 159, 146, 150]) // invalid utf-8 sequence
                .create();

            cli_tool
                .arg("rust")
                .args(["--server-url", &mock_server_base_url]);
            let result = cli_tool
                .output()
                .expect(error_messages::CMD_EXECUTION_FAILURE);

            let actual_error_message = String::from_utf8_lossy(&result.stderr);
            let expected_error_message =
                format!("{}\n", error_messages::INVALID_ENCODING);

            let actual_status_code = result.status.code();
            let expected_status_code = Some(exit_status::HTTP_CLIENT_ERROR);

            template_generator_mock.assert();

            assert_eq!(actual_status_code, expected_status_code);
            assert_eq!(actual_error_message, expected_error_message);
        }

        #[test]
        fn it_outputs_error_and_fails_when_body_parsing_issue_with_lister() {
            let mut cli_tool = get_test_bin(env!("CARGO_PKG_NAME"));

            let mut mock_server = Server::new();
            let mock_server_base_url = mock_server.url();
            let template_generator_mock = mock_server
                .mock("GET", constant::template_manager::LISTER_URI)
                .with_status(200)
                .with_body(vec![0, 159, 146, 150]) // invalid utf-8 sequence
                .create();

            cli_tool
                .arg("--list")
                .args(["--server-url", &mock_server_base_url]);
            let result = cli_tool
                .output()
                .expect(error_messages::CMD_EXECUTION_FAILURE);

            let actual_error_message = String::from_utf8_lossy(&result.stderr);
            let expected_error_message =
                format!("{}\n", error_messages::INVALID_ENCODING);

            let actual_status_code = result.status.code();
            let expected_status_code = Some(exit_status::HTTP_CLIENT_ERROR);

            template_generator_mock.assert();

            assert_eq!(actual_status_code, expected_status_code);
            assert_eq!(actual_error_message, expected_error_message);
        }
    }

    mod named_args {
        use std::{thread, time::Duration};

        use gitignore_template_generator::constant;

        use super::*;

        #[test]
        fn it_outputs_error_and_fails_when_generator_endpoint_not_found() {
            let mut cli_tool = get_test_bin(env!("CARGO_PKG_NAME"));

            let mut mock_server = Server::new();
            let mock_server_base_url = mock_server.url();
            let template_generator_service_uri =
                format!("{}/rust", template_manager::GENERATOR_URI);
            let template_generator_mock = mock_server
                .mock("GET", template_generator_service_uri.as_str())
                .with_status(404)
                .create();

            cli_tool
                .arg("rust")
                .args(["--server-url", &mock_server_base_url])
                .args(["--generator-uri", template_manager::GENERATOR_URI]);
            let result = cli_tool
                .output()
                .expect(error_messages::CMD_EXECUTION_FAILURE);

            let actual_error_message = String::from_utf8_lossy(&result.stderr);
            let expected_error_message = format!(
                "{}\n",
                error_messages::API_CALL_FAILURE
                    .replace("{error}", error_messages::HTTP_404)
            );

            let actual_status_code = result.status.code();
            let expected_status_code = Some(exit_status::GENERIC);

            template_generator_mock.assert();

            assert_eq!(actual_status_code, expected_status_code);
            assert_eq!(actual_error_message, expected_error_message);
        }

        #[test]
        fn it_outputs_error_and_fails_when_lister_endpoint_not_found() {
            let mut cli_tool = get_test_bin(env!("CARGO_PKG_NAME"));

            let mut mock_server = Server::new();
            let mock_server_base_url = mock_server.url();
            let template_generator_mock = mock_server
                .mock("GET", template_manager::LISTER_URI)
                .with_status(404)
                .create();

            cli_tool
                .arg("--list")
                .args(["--server-url", &mock_server_base_url])
                .args(["--lister-uri", template_manager::LISTER_URI]);
            let result = cli_tool
                .output()
                .expect(error_messages::CMD_EXECUTION_FAILURE);

            let actual_error_message = String::from_utf8_lossy(&result.stderr);
            let expected_error_message = format!(
                "{}\n",
                error_messages::API_CALL_FAILURE
                    .replace("{error}", error_messages::HTTP_404)
            );

            let actual_status_code = result.status.code();
            let expected_status_code = Some(exit_status::GENERIC);

            template_generator_mock.assert();

            assert_eq!(actual_status_code, expected_status_code);
            assert_eq!(actual_error_message, expected_error_message);
        }

        #[test]
        fn it_outputs_error_and_fails_when_timeout_reached() {
            let mut cli_tool = get_test_bin(env!("CARGO_PKG_NAME"));

            let mut mock_server = Server::new();
            let mock_server_base_url = mock_server.url();
            let template_generator_mock = mock_server
                .mock("GET", template_manager::LISTER_URI)
                .with_status(200)
                .with_chunked_body(|w| {
                    thread::sleep(Duration::from_secs(2));
                    w.write_all(b"rust\npython")
                })
                .create();

            cli_tool
                .arg("--list")
                .args(["--server-url", &mock_server_base_url])
                .args(["--lister-uri", template_manager::LISTER_URI])
                .args(["--timeout", "1"]);
            let result = cli_tool
                .output()
                .expect(error_messages::CMD_EXECUTION_FAILURE);

            let actual_error_message = String::from_utf8_lossy(&result.stderr);
            let expected_error_message =
                constant::error_messages::TIMEOUT.to_string() + "\n";

            let actual_status_code = result.status.code();
            let expected_status_code = Some(exit_status::HTTP_CLIENT_ERROR);

            template_generator_mock.assert();

            assert_eq!(actual_status_code, expected_status_code);
            assert_eq!(actual_error_message, expected_error_message);
        }
    }
}
