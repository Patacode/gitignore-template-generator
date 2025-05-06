use gitignore_template_generator::constant::{
    error_messages, exit_status, template_manager,
};
use mockito::Server;
use test_bin::get_test_bin;

mod success {
    use super::*;

    mod pos_args {
        use gitignore_template_generator::helper::load_expectation_file_as_string;

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
                .args(["--endpoint-uri", "/custom"]);
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
    }
}

mod failure {
    use super::*;

    mod pos_args {
        use super::*;

        #[test]
        fn it_outputs_error_and_fails_when_body_parsing_issue() {
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
                format!("{}\n", error_messages::BODY_PARSING_ISSUE);

            let actual_status_code = result.status.code();
            let expected_status_code = Some(exit_status::BODY_PARSING_ISSUE);

            template_generator_mock.assert();

            assert_eq!(actual_status_code, expected_status_code);
            assert_eq!(actual_error_message, expected_error_message);
        }
    }

    mod named_args {
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
                .args(["--endpoint-uri", template_manager::GENERATOR_URI]);
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
    }
}
