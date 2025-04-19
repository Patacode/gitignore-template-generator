use std::fs;

use crate::helper::*;
use gitignore_template_generator::constant::error_messages;
use rstest::*;
use test_bin::get_test_bin;

mod helper;

mod success {
    use super::*;

    mod pos_args {
        use super::*;

        #[rstest]
        #[case("rust", "rust_template")]
        #[case("rust python", "rust_python_template")]
        fn it_outputs_gitignore_templates_from_api(
            #[case] pos_args: &str,
            #[case] expectation_file_name: &str,
        ) {
            let mut cli_tool = get_test_bin(env!("CARGO_PKG_NAME"));

            cli_tool.args(parse_pos_args(pos_args));
            let result = cli_tool
                .output()
                .expect(error_messages::CMD_EXECUTION_FAILURE);

            let actual_output = parse_stdout(&result.stdout);
            let expected_output =
                load_expectation_file_as_string(expectation_file_name);

            assert!(result.status.success());
            assert_eq!(actual_output, expected_output);
        }
    }

    mod named_args {
        use super::*;

        #[test]
        fn it_outputs_version_infos_with_version_option() {
            let mut cli_tool = get_test_bin(env!("CARGO_PKG_NAME"));

            cli_tool.arg("-V");
            let result = cli_tool
                .output()
                .expect(error_messages::CMD_EXECUTION_FAILURE);

            let actual_output = parse_stdout(&result.stdout);
            let expected_output = format!(
                "{} {}\n",
                env!("CARGO_PKG_NAME"),
                env!("CARGO_PKG_VERSION"),
            );

            assert!(result.status.success());
            assert_eq!(actual_output, expected_output);
        }

        #[test]
        fn it_outputs_author_infos_with_author_option() {
            let mut cli_tool = get_test_bin(env!("CARGO_PKG_NAME"));

            cli_tool.arg("-a");
            let result = cli_tool
                .output()
                .expect(error_messages::CMD_EXECUTION_FAILURE);

            let actual_output = parse_stdout(&result.stdout);
            let expected_output = format!("{}\n", env!("CARGO_PKG_AUTHORS"));

            assert!(result.status.success());
            assert_eq!(actual_output, expected_output);
        }

        #[test]
        fn it_outputs_help_infos_with_help_option() {
            let mut cli_tool = get_test_bin(env!("CARGO_PKG_NAME"));

            cli_tool.arg("-h");
            let result = cli_tool
                .output()
                .expect(error_messages::CMD_EXECUTION_FAILURE);

            let actual_output = parse_stdout(&result.stdout);
            let expected_output =
                load_expectation_file_as_string("help_message")
                    .replace("{version}", env!("CARGO_PKG_VERSION"))
                    .replace("{author}", env!("CARGO_PKG_AUTHORS"));

            assert!(result.status.success());
            assert_eq!(actual_output, expected_output);
        }
    }
}

mod failure {
    use super::*;

    mod pos_args {
        use super::*;

        #[test]
        fn it_outputs_error_and_fails_when_no_pos_args_provided() {
            let mut cmd = get_test_bin(env!("CARGO_PKG_NAME"));

            let output =
                cmd.output().expect(error_messages::CMD_EXECUTION_FAILURE);
            let expected =
                fs::read_to_string("tests/expected/no_pos_args_error.txt")
                    .expect(error_messages::FILE_READ_TO_STRING_FAILURE);
            let actual = String::from_utf8_lossy(&output.stderr);

            assert!(!output.status.success());
            assert_eq!(output.status.code(), Some(2));
            assert_eq!(actual, expected);
        }

        #[test]
        fn it_outputs_error_and_fails_when_commas_in_pos_args() {
            let mut cmd = get_test_bin(env!("CARGO_PKG_NAME"));

            cmd.args(["rust", "python,java"]);

            let output =
                cmd.output().expect(error_messages::CMD_EXECUTION_FAILURE);
            let expected =
                fs::read_to_string("tests/expected/comma_pos_args_error.txt")
                    .expect(error_messages::FILE_READ_TO_STRING_FAILURE);
            let actual = String::from_utf8_lossy(&output.stderr);

            assert!(!output.status.success());
            assert_eq!(output.status.code(), Some(2));
            assert_eq!(actual, expected);
        }

        #[test]
        fn it_outputs_error_and_fails_when_template_not_found() {
            let mut cmd = get_test_bin(env!("CARGO_PKG_NAME"));

            cmd.args(["foo"]);

            let output =
                cmd.output().expect(error_messages::CMD_EXECUTION_FAILURE);
            let expected =
                "An error occurred during the API call: http status: 404\n";
            let actual = String::from_utf8_lossy(&output.stderr);

            assert!(!output.status.success());
            assert_eq!(output.status.code(), Some(2));
            assert_eq!(actual, expected);
        }
    }
}
