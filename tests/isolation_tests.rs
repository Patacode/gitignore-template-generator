use gitignore_template_generator::constant::{
    error_messages, exit_status, template_generator,
};
use mockito::Server;
use test_bin::get_test_bin;

mod failure {
    use super::*;

    mod pos_args {
        use super::*;

        #[test]
        fn it_outputs_error_and_fails_when_body_parsing_issue() {
            let mut cli_tool = get_test_bin(env!("CARGO_PKG_NAME"));

            let mut mock_server = Server::new();
            let mock_server_base_url = mock_server.url();
            let uri = format!("{}/rust", template_generator::URI);
            let mock = mock_server
                .mock("GET", uri.as_str())
                .with_status(200)
                .with_body(vec![0, 159, 146, 150])
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

            mock.assert();

            assert_eq!(actual_status_code, expected_status_code);
            assert_eq!(actual_error_message, expected_error_message);
        }
    }
}
