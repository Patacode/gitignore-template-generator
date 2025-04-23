use gitignore_template_generator::constant;
use gitignore_template_generator::helper::*;
use rstest::*;
use test_bin::get_test_bin;

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
                .expect(constant::error_messages::CMD_EXECUTION_FAILURE);

            let actual_output = parse_bytes(&result.stdout);
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

            cli_tool.arg(format!("-{}", constant::cli_options::VERSION.short));
            let result = cli_tool
                .output()
                .expect(constant::error_messages::CMD_EXECUTION_FAILURE);

            let actual_output = parse_bytes(&result.stdout);
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

            cli_tool.arg(format!("-{}", constant::cli_options::AUTHOR.short));
            let result = cli_tool
                .output()
                .expect(constant::error_messages::CMD_EXECUTION_FAILURE);

            let actual_output = parse_bytes(&result.stdout);
            let expected_output = format!("{}\n", env!("CARGO_PKG_AUTHORS"));

            assert!(result.status.success());
            assert_eq!(actual_output, expected_output);
        }

        #[test]
        fn it_outputs_help_infos_with_help_option() {
            let mut cli_tool = get_test_bin(env!("CARGO_PKG_NAME"));

            cli_tool.arg(format!("-{}", constant::cli_options::HELP.short));
            let result = cli_tool
                .output()
                .expect(constant::error_messages::CMD_EXECUTION_FAILURE);

            let actual_output = parse_bytes(&result.stdout);
            let expected_output = get_ansi_help_message() + "\n";

            assert!(result.status.success());
            assert_eq!(actual_output, expected_output);
        }
    }
}

mod failure {
    use super::*;

    mod pos_args {
        use super::*;

        #[rstest]
        #[case("", "ansi_no_pos_args_error")]
        #[case("rust python,java", "ansi_comma_pos_args_error")]
        #[case("foo", "template_not_found_error")]
        fn it_outputs_error_and_fails_when_invalid_pos_args(
            #[case] pos_args: &str,
            #[case] expectation_file_name: &str,
        ) {
            let mut cli_tools = get_test_bin(env!("CARGO_PKG_NAME"));

            cli_tools.args(parse_pos_args(pos_args));
            let result = cli_tools
                .output()
                .expect(constant::error_messages::CMD_EXECUTION_FAILURE);

            let actual_output = parse_bytes(&result.stderr);
            let expected_output =
                load_expectation_file_as_string(expectation_file_name) + "\n";

            let actual_status_code = result.status.code();
            let expected_status_code = Some(constant::exit_status::GENERIC);

            assert_eq!(actual_status_code, expected_status_code);
            assert_eq!(actual_output, expected_output);
        }
    }
}
