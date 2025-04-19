use gitignore_template_generator::constant::{error_messages, exit_status};
use mockito::Server;
use test_bin::get_test_bin;

mod failure {
    use super::*;

    mod pos_args {
        use super::*;

        #[test]
        fn it_outputs_error_and_fails_when_body_parsing_issue() {
            let mut cmd = get_test_bin(env!("CARGO_PKG_NAME"));

            let mut server = Server::new();
            let base_url = server.url();
            let uri = "/developers/gitignore/api/rust";

            let mock = server
                .mock("GET", uri)
                .with_status(200)
                .with_body(vec![0, 159, 146, 150])
                .create();

            cmd.arg("rust").args(["--server-url", &base_url]);

            let output =
                cmd.output().expect(error_messages::CMD_EXECUTION_FAILURE);
            let expected = format!("{}\n", error_messages::BODY_PARSING_ISSUE);
            let actual = String::from_utf8_lossy(&output.stderr);

            mock.assert();

            assert_eq!(
                output.status.code(),
                Some(exit_status::BODY_PARSING_ISSUE)
            );
            assert_eq!(actual, expected);
        }
    }
}
