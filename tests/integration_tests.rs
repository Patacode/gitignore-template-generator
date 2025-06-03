use gitignore_template_generator::{constant, helper::*};
use rstest::*;
use serial_test::parallel;
#[cfg(feature = "local_templating")]
use serial_test::serial;
use test_bin::get_test_bin;

mod success {
    use super::*;

    mod pos_args {
        use super::*;

        cfg_if::cfg_if! {
            if #[cfg(feature = "local_templating")] {
                #[rstest]
                #[serial]
                #[case("rust", "local_remote_rust_template")]
                #[case("rust python", "local_real_remote_python_rust_template")]
                fn it_outputs_gitignore_templates_from_api(
                    #[case] pos_args: &str,
                    #[case] expectation_file_name: &str,
                ) {
                    let mut cli_tool = get_test_bin(env!("CARGO_PKG_NAME"));
                    let template_dir = get_resource_path("templates");

                    unsafe {
                        std::env::set_var(
                            constant::template_manager::HOME_ENV_VAR,
                            &template_dir,
                        );
                    }

                    cli_tool.args(parse_pos_args(pos_args));
                    let result = cli_tool
                        .output()
                        .expect(constant::error_messages::CMD_EXECUTION_FAILURE);

                    let actual_output = parse_bytes(&result.stdout);
                    let expected_output =
                        load_expectation_file_as_string(expectation_file_name);

                    unsafe {
                        std::env::remove_var(
                            constant::template_manager::HOME_ENV_VAR,
                        );
                    }

                    assert!(result.status.success());
                    assert_eq!(actual_output, expected_output);
                }
            } else {
                #[rstest]
                #[parallel]
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
        }
    }

    mod named_args {
        use super::*;

        #[test]
        #[parallel]
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
        #[parallel]
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
        #[parallel]
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

        cfg_if::cfg_if! {
            if #[cfg(feature = "local_templating")] {
                #[test]
                #[serial]
                fn it_outputs_available_template_list_from_api_with_list_option() {
                    let mut cli_tool = get_test_bin(env!("CARGO_PKG_NAME"));
                    let template_dir = get_resource_path("templates");

                    unsafe {
                        std::env::set_var(
                            constant::template_manager::HOME_ENV_VAR,
                            &template_dir,
                        );
                    }

                    cli_tool.arg(format!("-{}", constant::cli_options::LIST.short));
                    let result = cli_tool
                        .output()
                        .expect(constant::error_messages::CMD_EXECUTION_FAILURE);

                    let actual_output = parse_bytes(&result.stdout);
                    let expected_output =
                        load_expectation_file_as_string("local_remote_template_list");

                    unsafe {
                        std::env::remove_var(
                            constant::template_manager::HOME_ENV_VAR,
                        );
                    }

                    assert!(result.status.success());
                    assert_eq!(actual_output, expected_output);
                }
            } else {
                #[test]
                #[parallel]
                fn it_outputs_available_template_list_from_api_with_list_option() {
                    let mut cli_tool = get_test_bin(env!("CARGO_PKG_NAME"));

                    cli_tool.arg(format!("-{}", constant::cli_options::LIST.short));
                    let result = cli_tool
                        .output()
                        .expect(constant::error_messages::CMD_EXECUTION_FAILURE);

                    let actual_output = parse_bytes(&result.stdout);
                    let expected_output =
                        load_expectation_file_as_string("template_list");

                    assert!(result.status.success());
                    assert_eq!(actual_output, expected_output);
                }
            }
        }

        cfg_if::cfg_if! {
            if #[cfg(feature = "local_templating")] {
                #[test]
                #[serial]
                fn it_outputs_gitignore_templates_from_api_with_check_option() {
                    let mut cli_tool = get_test_bin(env!("CARGO_PKG_NAME"));
                    let template_dir = get_resource_path("templates");

                    unsafe {
                        std::env::set_var(
                            constant::template_manager::HOME_ENV_VAR,
                            &template_dir,
                        );
                    }

                    cli_tool.args(parse_pos_args("rust python --check"));
                    let result = cli_tool
                        .output()
                        .expect(constant::error_messages::CMD_EXECUTION_FAILURE);

                    let actual_output = parse_bytes(&result.stdout);
                    let expected_output =
                        load_expectation_file_as_string("local_real_remote_python_rust_template");

                    unsafe {
                        std::env::remove_var(
                            constant::template_manager::HOME_ENV_VAR,
                        );
                    }

                    assert!(result.status.success());
                    assert_eq!(actual_output, expected_output);
                }
            } else {
                #[test]
                #[parallel]
                fn it_outputs_gitignore_templates_from_api_with_check_option() {
                    let mut cli_tool = get_test_bin(env!("CARGO_PKG_NAME"));

                    cli_tool.args(parse_pos_args("rust python --check"));
                    let result = cli_tool
                        .output()
                        .expect(constant::error_messages::CMD_EXECUTION_FAILURE);

                    let actual_output = parse_bytes(&result.stdout);
                    let expected_output =
                        load_expectation_file_as_string("rust_python_template");

                    assert!(result.status.success());
                    assert_eq!(actual_output, expected_output);
                }
            }
        }

        cfg_if::cfg_if! {
            if #[cfg(feature = "local_templating")] {
                #[test]
                #[serial]
                fn it_outputs_gitignore_templates_from_api_with_timeout_option() {
                    let mut cli_tool = get_test_bin(env!("CARGO_PKG_NAME"));
                    let template_dir = get_resource_path("templates");

                    unsafe {
                        std::env::set_var(
                            constant::template_manager::HOME_ENV_VAR,
                            &template_dir,
                        );
                    }

                    cli_tool.args(parse_pos_args("rust python --timeout 5"));
                    let result = cli_tool
                        .output()
                        .expect(constant::error_messages::CMD_EXECUTION_FAILURE);

                    let actual_output = parse_bytes(&result.stdout);
                    let expected_output =
                        load_expectation_file_as_string("local_real_remote_python_rust_template");

                    unsafe {
                        std::env::remove_var(
                            constant::template_manager::HOME_ENV_VAR,
                        );
                    }

                    assert!(result.status.success());
                    assert_eq!(actual_output, expected_output);
                }
            } else {
                #[test]
                #[parallel]
                fn it_outputs_gitignore_templates_from_api_with_timeout_option() {
                    let mut cli_tool = get_test_bin(env!("CARGO_PKG_NAME"));

                    cli_tool.args(parse_pos_args("rust python --timeout 5"));
                    let result = cli_tool
                        .output()
                        .expect(constant::error_messages::CMD_EXECUTION_FAILURE);

                    let actual_output = parse_bytes(&result.stdout);
                    let expected_output =
                        load_expectation_file_as_string("rust_python_template");

                    assert!(result.status.success());
                    assert_eq!(actual_output, expected_output);
                }
            }
        }

        cfg_if::cfg_if! {
            if #[cfg(feature = "local_templating")] {
                #[test]
                #[serial]
                fn it_outputs_gitignore_templates_from_api_with_timeout_unit_option() {
                    let mut cli_tool = get_test_bin(env!("CARGO_PKG_NAME"));
                    let template_dir = get_resource_path("templates");

                    unsafe {
                        std::env::set_var(
                            constant::template_manager::HOME_ENV_VAR,
                            &template_dir,
                        );
                    }

                    cli_tool.args(parse_pos_args(
                        "rust python --timeout 5000 --timeout-unit millisecond",
                    ));
                    let result = cli_tool
                        .output()
                        .expect(constant::error_messages::CMD_EXECUTION_FAILURE);

                    let actual_output = parse_bytes(&result.stdout);
                    let expected_output =
                        load_expectation_file_as_string("local_real_remote_python_rust_template");

                    unsafe {
                        std::env::remove_var(
                            constant::template_manager::HOME_ENV_VAR,
                        );
                    }

                    assert!(result.status.success());
                    assert_eq!(actual_output, expected_output);
                }
            } else {
                #[test]
                #[parallel]
                fn it_outputs_gitignore_templates_from_api_with_timeout_unit_option() {
                    let mut cli_tool = get_test_bin(env!("CARGO_PKG_NAME"));

                    cli_tool.args(parse_pos_args(
                        "rust python --timeout 5000 --timeout-unit millisecond",
                    ));
                    let result = cli_tool
                        .output()
                        .expect(constant::error_messages::CMD_EXECUTION_FAILURE);

                    let actual_output = parse_bytes(&result.stdout);
                    let expected_output =
                        load_expectation_file_as_string("rust_python_template");

                    assert!(result.status.success());
                    assert_eq!(actual_output, expected_output);
                }
            }
        }
    }
}

mod failure {
    use super::*;

    mod pos_args {
        use super::*;

        cfg_if::cfg_if! {
            if #[cfg(feature = "local_templating")] {
                #[rstest]
                #[serial]
                #[case("", "ansi_no_pos_args_error")]
                #[case("rust python,java", "ansi_comma_pos_args_error")]
                #[case("foo", "local_remote_template_not_found_error")]
                fn it_outputs_error_and_fails_when_invalid_pos_args(
                    #[case] pos_args: &str,
                    #[case] expectation_file_name: &str,
                ) {
                    let mut cli_tools = get_test_bin(env!("CARGO_PKG_NAME"));
                    let template_dir = get_resource_path("templates");

                    unsafe {
                        std::env::set_var(
                            constant::template_manager::HOME_ENV_VAR,
                            &template_dir,
                        );
                    }

                    cli_tools.args(parse_pos_args(pos_args));
                    let result = cli_tools
                        .output()
                        .expect(constant::error_messages::CMD_EXECUTION_FAILURE);

                    let actual_output = parse_bytes(&result.stderr);
                    let expected_output =
                        load_expectation_file_as_string(expectation_file_name) + "\n";

                    let actual_status_code = result.status.code();
                    let expected_status_code = Some(constant::exit_status::GENERIC);

                    unsafe {
                        std::env::remove_var(
                            constant::template_manager::HOME_ENV_VAR,
                        );
                    }

                    assert_eq!(actual_status_code, expected_status_code);
                    assert_eq!(actual_output, expected_output);
                }
            } else {
                #[rstest]
                #[parallel]
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
    }

    mod named_args {
        use super::*;

        mod failure {
            use super::*;

            #[test]
            #[parallel]
            fn it_outputs_error_and_fails_when_server_not_found() {
                let mut cli_tools = get_test_bin(env!("CARGO_PKG_NAME"));

                cli_tools
                    .args(parse_pos_args("-s https://fjizefhize.com rust"));
                let result = cli_tools
                    .output()
                    .expect(constant::error_messages::CMD_EXECUTION_FAILURE);

                let actual_output = parse_bytes(&result.stderr);
                let expected_output =
                    load_expectation_file_as_string("server_not_found_error")
                        + "\n";

                let actual_status_code = result.status.code();
                let expected_status_code = Some(constant::exit_status::GENERIC);

                assert_eq!(actual_status_code, expected_status_code);
                assert_eq!(actual_output, expected_output);
            }

            cfg_if::cfg_if! {
                if #[cfg(feature = "local_templating")] {
                    #[test]
                    #[serial]
                    fn it_outputs_error_and_fails_when_inexistent_templates() {
                        let mut cli_tools = get_test_bin(env!("CARGO_PKG_NAME"));
                        let template_dir = get_resource_path("templates");

                        unsafe {
                            std::env::set_var(
                                constant::template_manager::HOME_ENV_VAR,
                                &template_dir,
                            );
                        }

                        cli_tools.args(parse_pos_args("rust pyth foo --check"));
                        let result = cli_tools
                            .output()
                            .expect(constant::error_messages::CMD_EXECUTION_FAILURE);

                        let actual_output = parse_bytes(&result.stderr);
                        let expected_output = load_expectation_file_as_string(
                            "inexistent_templates_error",
                        ) + "\n";

                        let actual_status_code = result.status.code();
                        let expected_status_code = Some(constant::exit_status::GENERIC);

                        unsafe {
                            std::env::remove_var(
                                constant::template_manager::HOME_ENV_VAR,
                            );
                        }

                        assert_eq!(actual_status_code, expected_status_code);
                        assert_eq!(actual_output, expected_output);
                    }
                } else {
                    #[test]
                    #[parallel]
                    fn it_outputs_error_and_fails_when_inexistent_templates() {
                        let mut cli_tools = get_test_bin(env!("CARGO_PKG_NAME"));

                        cli_tools.args(parse_pos_args("rust pyth foo --check"));
                        let result = cli_tools
                            .output()
                            .expect(constant::error_messages::CMD_EXECUTION_FAILURE);

                        let actual_output = parse_bytes(&result.stderr);
                        let expected_output = load_expectation_file_as_string(
                            "inexistent_templates_error",
                        ) + "\n";

                        let actual_status_code = result.status.code();
                        let expected_status_code = Some(constant::exit_status::GENERIC);

                        assert_eq!(actual_status_code, expected_status_code);
                        assert_eq!(actual_output, expected_output);
                    }
                }
            }
        }
    }
}
