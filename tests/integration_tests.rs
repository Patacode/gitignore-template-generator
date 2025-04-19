use std::fs;

use gitignore_template_generator::constant::{self, error_messages, path};
use regex::Regex;
use test_bin::get_test_bin;

mod success {
    use super::*;

    mod pos_args {
        use super::*;

        #[test]
        fn it_outputs_template_with_one_pos_arg() {
            let mut cli_tool = get_test_bin(env!("CARGO_PKG_NAME"));

            let expectation_file_name = "rust_template";
            let expectation_file_path = format!(
                "{}/{expectation_file_name}.txt",
                path::TEST_EXPECTATIONS
            );

            cli_tool.arg("rust");
            let result = cli_tool
                .output()
                .expect(error_messages::CMD_EXECUTION_FAILURE);

            let actual_output = String::from_utf8_lossy(&result.stdout);
            let expected_output = fs::read_to_string(expectation_file_path)
                .expect(error_messages::FILE_READ_TO_STRING_FAILURE);

            assert!(result.status.success());
            assert_eq!(actual_output, expected_output);
        }

        #[test]
        fn it_outputs_combined_templates_with_multiple_pos_args() {
            let mut cli_tool = get_test_bin(env!("CARGO_PKG_NAME"));

            let expectation_file_name = "rust_python_template";
            let expectation_file_path = format!(
                "{}/{expectation_file_name}.txt",
                path::TEST_EXPECTATIONS
            );

            cli_tool.args(["rust", "python"]);
            let result = cli_tool
                .output()
                .expect(error_messages::CMD_EXECUTION_FAILURE);

            let actual_output = String::from_utf8_lossy(&result.stdout);
            let expected_output = fs::read_to_string(expectation_file_path)
                .expect(error_messages::FILE_READ_TO_STRING_FAILURE);

            assert!(result.status.success());
            assert_eq!(actual_output, expected_output);
        }
    }

    mod named_args {
        use super::*;

        #[test]
        fn it_outputs_version_infos_with_version_option() {
            let mut cli_tool = get_test_bin(env!("CARGO_PKG_NAME"));

            let expected_output_pattern = format!(
                r"^{} {}\n$",
                env!("CARGO_PKG_NAME"),
                constant::regex::SEMVER_VERSION,
            );

            cli_tool.arg("-V");
            let result = cli_tool
                .output()
                .expect(error_messages::CMD_EXECUTION_FAILURE);

            let actual_output = String::from_utf8_lossy(&result.stdout);
            let expected_output_pattern =
                Regex::new(&expected_output_pattern).unwrap();

            assert!(result.status.success());
            assert!(
                expected_output_pattern.is_match(&actual_output),
                "{}",
                error_messages::REGEX_NO_MATCH
                    .replace("{actual}", &actual_output)
                    .replace("{expected}", expected_output_pattern.as_str())
            );
        }

        #[test]
        fn it_outputs_author_infos_with_author_option() {
            let mut cli_tool = get_test_bin(env!("CARGO_PKG_NAME"));

            cli_tool.arg("-a");
            let result = cli_tool
                .output()
                .expect(error_messages::CMD_EXECUTION_FAILURE);

            let actual_output = String::from_utf8_lossy(&result.stdout);
            let expected_output = format!("{}\n", env!("CARGO_PKG_AUTHORS"));

            assert!(result.status.success());
            assert_eq!(actual_output, expected_output);
        }

        #[test]
        fn it_outputs_help_infos_with_help_option() {
            let mut cli_tool = get_test_bin(env!("CARGO_PKG_NAME"));

            let expectation_file_name = "help_message";
            let expectation_file_path = format!(
                "{}/{expectation_file_name}.txt",
                path::TEST_EXPECTATIONS
            );

            cli_tool.arg("-h");
            let result = cli_tool
                .output()
                .expect(error_messages::CMD_EXECUTION_FAILURE);

            let actual_output = String::from_utf8_lossy(&result.stdout);
            let expected_output = fs::read_to_string(expectation_file_path)
                .expect(error_messages::FILE_READ_TO_STRING_FAILURE)
                .replace("{version}", env!("CARGO_PKG_VERSION"));

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
