use std::{
    collections::HashMap,
    fs::{self, File},
    os::unix::fs::PermissionsExt,
    path::Path,
};

use rstest::{fixture, rstest};
use serial_test::{parallel, serial};

use super::*;
use crate::{
    constant,
    core::impls::{
        LocalGitignoreTemplateManager, RemoteGitignoreTemplateManager,
    },
    helper::{self, make_string_vec},
    http_client::{MockEndpointHttpClient, MockHttpClient},
    test_helper::{EnvTestContext, create_env_test_context, set_env_var},
};

#[fixture]
fn ctx() -> EnvTestContext {
    create_env_test_context()
}

mod local_gitignore_template_manager {

    use super::*;

    mod generate {
        use super::*;

        mod success {
            use super::*;

            #[rstest]
            #[serial]
            fn it_generates_template_from_local_fs_using_env_var(
                _ctx: EnvTestContext,
            ) {
                let template_dir = helper::get_resource_path("templates");
                let template_names = make_string_vec("python rust");

                set_env_var(
                    constant::template_manager::HOME_ENV_VAR,
                    &template_dir,
                );

                let generator =
                    LocalGitignoreTemplateManager::new(Some(&template_dir));

                let expected_template = Ok(QualifiedString {
                    value: helper::load_expectation_file_as_string(
                        "local_rust_python_template",
                    ),
                    kind: StringKind::Local,
                });
                let actual_template = generator.generate(&template_names);

                assert_eq!(actual_template, expected_template);
            }

            #[rstest]
            #[serial]
            fn it_generates_template_from_local_fs_using_given_dir(
                _ctx: EnvTestContext,
            ) {
                let template_dir = helper::get_resource_path("templates");
                let template_names = make_string_vec("python rust");
                let generator =
                    LocalGitignoreTemplateManager::new(Some(&template_dir));

                let expected_template = QualifiedString {
                    value: helper::load_expectation_file_as_string(
                        "local_rust_python_template",
                    ),
                    kind: StringKind::Local,
                };
                let actual_template = generator.generate(&template_names);

                assert!(actual_template.is_ok());

                let actual_template = actual_template.unwrap();
                assert_eq!(actual_template, expected_template);
            }

            #[rstest]
            #[serial]
            fn it_generates_empty_template_when_no_template_names(
                _ctx: EnvTestContext,
            ) {
                let template_dir = helper::get_resource_path("templates");
                let generator =
                    LocalGitignoreTemplateManager::new(Some(&template_dir));

                let expected_template = QualifiedString {
                    value: String::new(),
                    kind: StringKind::Local,
                };
                let actual_template = generator.generate(&[]);

                assert!(actual_template.is_ok());

                let actual_template = actual_template.unwrap();
                assert_eq!(actual_template, expected_template);
            }
        }

        mod failure {
            use super::*;

            #[rstest]
            #[serial]
            fn it_fails_when_unsupported_template_names(_ctx: EnvTestContext) {
                let template_dir = helper::get_resource_path("templates");
                let template_names = make_string_vec("python rust unknown");
                let generator =
                    LocalGitignoreTemplateManager::new(Some(&template_dir));

                let expected_error = ProgramExit {
                    message: format!(
                        "{}: {}",
                        constant::error_messages::LOCAL_GENERATION,
                        constant::error_messages::UNSUPPORTED_TEMPLATE
                    ),
                    exit_status: constant::exit_status::GENERIC,
                    styled_message: None,
                    kind: ExitKind::Error,
                };
                let actual_error = generator.generate(&template_names);

                assert!(actual_error.is_err());

                let actual_error = actual_error.unwrap_err();
                assert_eq!(actual_error, expected_error);
            }

            #[rstest]
            #[serial]
            fn it_propagates_fs_error_if_any_occurred(_ctx: EnvTestContext) {
                let template_dir = helper::get_resource_path("templates");
                let template_names = make_string_vec("dummy");
                let generator =
                    LocalGitignoreTemplateManager::new(Some(&template_dir));

                let expected_error = ProgramExit {
                    message: format!(
                        "{}: Is a directory (os error 21)",
                        constant::error_messages::LOCAL_GENERATION
                    ),
                    exit_status: constant::exit_status::GENERIC,
                    styled_message: None,
                    kind: ExitKind::Error,
                };
                let actual_error = generator.generate(&template_names);

                assert!(actual_error.is_err());

                let actual_error = actual_error.unwrap_err();
                assert_eq!(actual_error, expected_error);
            }
        }
    }

    mod generate_with_template_check {
        use super::*;

        mod success {
            use super::*;

            #[rstest]
            #[serial]
            fn it_generates_template_from_local_fs_using_env_var(
                _ctx: EnvTestContext,
            ) {
                let template_dir = helper::get_resource_path("templates");
                let template_names = make_string_vec("python rust");
                let generator =
                    LocalGitignoreTemplateManager::new(Some(&template_dir));

                set_env_var(
                    constant::template_manager::HOME_ENV_VAR,
                    &template_dir,
                );

                let expected_template = QualifiedString {
                    value: helper::load_expectation_file_as_string(
                        "local_rust_python_template",
                    ),
                    kind: StringKind::Local,
                };
                let actual_template =
                    generator.generate_with_template_check(&template_names);

                assert!(actual_template.is_ok());

                let actual_template = actual_template.unwrap();
                assert_eq!(actual_template, expected_template);
            }

            #[rstest]
            #[serial]
            fn it_generates_template_from_local_fs_using_given_dir(
                _ctx: EnvTestContext,
            ) {
                let template_dir = helper::get_resource_path("templates");
                let template_names = make_string_vec("python rust");
                let generator =
                    LocalGitignoreTemplateManager::new(Some(&template_dir));

                let expected_template = QualifiedString {
                    value: helper::load_expectation_file_as_string(
                        "local_rust_python_template",
                    ),
                    kind: StringKind::Local,
                };
                let actual_template =
                    generator.generate_with_template_check(&template_names);

                assert!(actual_template.is_ok());

                let actual_template = actual_template.unwrap();
                assert_eq!(actual_template, expected_template);
            }
        }

        mod failure {
            use super::*;

            #[rstest]
            #[serial]
            fn it_fails_with_detailed_msg_when_unsupported_template_names(
                _ctx: EnvTestContext,
            ) {
                let template_dir = helper::get_resource_path("templates");
                let template_names = make_string_vec("python rust unknown");
                let generator =
                    LocalGitignoreTemplateManager::new(Some(&template_dir));

                let expected_error = ProgramExit {
                    message:
                        constant::error_messages::INEXISTENT_TEMPLATE_NAMES
                            .replace("{templates}", "unknown"),
                    exit_status: constant::exit_status::GENERIC,
                    styled_message: None,
                    kind: ExitKind::Error,
                };
                let actual_error =
                    generator.generate_with_template_check(&template_names);

                assert!(actual_error.is_err());

                let actual_error = actual_error.unwrap_err();
                assert_eq!(actual_error, expected_error);
            }

            #[rstest]
            #[serial]
            fn it_propagates_fs_error_if_any_occurred(_ctx: EnvTestContext) {
                let template_dir = helper::get_resource_path("templates/dummy");
                let template_names = make_string_vec("foo");
                let file_path = format!("{template_dir}/foo.txt");
                let file_path = Path::new(&file_path);
                let file = File::create(file_path).unwrap();
                let generator =
                    LocalGitignoreTemplateManager::new(Some(&template_dir));

                let mut perms = file.metadata().unwrap().permissions();
                perms.set_mode(0o264);
                fs::set_permissions(file_path, perms).unwrap();

                let expected_error = ProgramExit {
                    message: format!(
                        "{}: Permission denied (os error 13)",
                        constant::error_messages::LOCAL_GENERATION
                    ),
                    exit_status: constant::exit_status::GENERIC,
                    styled_message: None,
                    kind: ExitKind::Error,
                };
                let actual_error =
                    generator.generate_with_template_check(&template_names);

                assert!(actual_error.is_err());

                let actual_error = actual_error.unwrap_err();
                assert_eq!(actual_error, expected_error);
            }
        }
    }

    mod list {
        use super::*;

        mod success {
            use super::*;

            #[rstest]
            #[serial]
            fn it_lists_templates_from_local_fs_using_env_var(
                _ctx: EnvTestContext,
            ) {
                let template_dir = helper::get_resource_path("templates");
                let lister =
                    LocalGitignoreTemplateManager::new(Some(&template_dir));

                set_env_var(
                    constant::template_manager::HOME_ENV_VAR,
                    &template_dir,
                );

                let expected_list = QualifiedString {
                    value: "python\nrust".to_string(),
                    kind: StringKind::Local,
                };
                let actual_list = lister.list();
                assert!(actual_list.is_ok());

                let actual_list = actual_list.unwrap();
                assert_eq!(actual_list, expected_list);
            }

            #[rstest]
            #[serial]
            fn it_lists_templates_from_local_fs_using_given_dir(
                _ctx: EnvTestContext,
            ) {
                let template_dir = helper::get_resource_path("templates");
                let lister =
                    LocalGitignoreTemplateManager::new(Some(&template_dir));

                let expected_list = QualifiedString {
                    value: "python\nrust".to_string(),
                    kind: StringKind::Local,
                };
                let actual_list = lister.list();

                assert!(actual_list.is_ok());

                let actual_list = actual_list.unwrap();
                assert_eq!(actual_list, expected_list);
            }

            #[rstest]
            #[serial]
            fn it_returns_empty_string_when_inexistent_dir(
                _ctx: EnvTestContext,
            ) {
                let template_dir = helper::get_resource_path("inexistent");
                let lister =
                    LocalGitignoreTemplateManager::new(Some(&template_dir));

                let expected_list = QualifiedString {
                    value: String::new(),
                    kind: StringKind::Local,
                };
                let actual_list = lister.list();

                assert!(actual_list.is_ok());

                let actual_list = actual_list.unwrap();
                assert_eq!(actual_list, expected_list);
            }
        }

        mod failure {
            use super::*;

            #[rstest]
            #[serial]
            fn it_propagates_fs_error_if_any_occurred(_ctx: EnvTestContext) {
                let template_dir =
                    helper::get_resource_path("templates/python.txt");
                let lister =
                    LocalGitignoreTemplateManager::new(Some(&template_dir));

                let expected_error = ProgramExit {
                    message: format!(
                        "{}: Not a directory (os error 20)",
                        constant::error_messages::LOCAL_LISTING
                    ),
                    exit_status: constant::exit_status::GENERIC,
                    styled_message: None,
                    kind: ExitKind::Error,
                };
                let actual_error = lister.list();

                assert!(actual_error.is_err());

                let actual_error = actual_error.unwrap_err();
                assert_eq!(actual_error, expected_error);
            }
        }
    }
}

mod remote_gitignore_template_manager {
    use super::*;

    mod generate {
        use super::*;

        mod success {
            use super::*;

            #[test]
            #[parallel]
            fn it_generates_template_using_provided_client() {
                let template_names = make_string_vec("rust python");
                let generated_template = "all good";
                let http_client = MockHttpClient {
                    response: Ok(String::from(generated_template)),
                };
                let generator = RemoteGitignoreTemplateManager::new(
                    Box::new(http_client),
                    Some(constant::template_manager::GENERATOR_URI),
                    Some(constant::template_manager::LISTER_URI),
                );

                let actual = generator.generate(&template_names);
                let expected: Result<QualifiedString, ProgramExit> =
                    Ok(QualifiedString {
                        value: String::from(generated_template),
                        kind: StringKind::Remote,
                    });

                assert_eq!(actual, expected);
            }

            #[test]
            #[parallel]
            fn it_works_with_none_endpoint_uri() {
                let template_names = make_string_vec("rust python");
                let generated_template = "all good";
                let http_client = MockHttpClient {
                    response: Ok(String::from(generated_template)),
                };
                let generator = RemoteGitignoreTemplateManager::new(
                    Box::new(http_client),
                    None,
                    Some(constant::template_manager::LISTER_URI),
                );

                let actual = generator.generate(&template_names);
                let expected: Result<QualifiedString, ProgramExit> =
                    Ok(QualifiedString {
                        value: String::from(generated_template),
                        kind: StringKind::Remote,
                    });

                assert_eq!(actual, expected);
            }

            #[test]
            #[parallel]
            fn it_generates_empty_template_when_no_template_names() {
                let http_client = MockHttpClient {
                    response: Ok(String::from("all good")),
                };
                let generator = RemoteGitignoreTemplateManager::new(
                    Box::new(http_client),
                    None,
                    Some(constant::template_manager::LISTER_URI),
                );

                let expected_template = QualifiedString {
                    value: String::new(),
                    kind: StringKind::Remote,
                };
                let actual_template = generator.generate(&[]);

                assert!(actual_template.is_ok());

                let actual_template = actual_template.unwrap();
                assert_eq!(actual_template, expected_template);
            }
        }

        mod failure {
            use super::*;

            #[test]
            #[parallel]
            fn it_propagates_error_from_client_if_any() {
                let template_names = make_string_vec("rust pyth");
                let error_message = "all bad";
                let http_client = MockHttpClient {
                    response: Err(ProgramExit {
                        message: String::from(error_message),
                        exit_status: constant::exit_status::GENERIC,
                        styled_message: None,
                        kind: ExitKind::Error,
                    }),
                };
                let generator = RemoteGitignoreTemplateManager::new(
                    Box::new(http_client),
                    Some(constant::template_manager::GENERATOR_URI),
                    Some(constant::template_manager::LISTER_URI),
                );

                let actual = generator.generate(&template_names);
                let expected: Result<QualifiedString, ProgramExit> =
                    Err(ProgramExit {
                        message: String::from(error_message),
                        exit_status: constant::exit_status::GENERIC,
                        styled_message: None,
                        kind: ExitKind::Error,
                    });

                assert_eq!(actual, expected);
            }
        }
    }

    mod generate_with_template_check {
        use super::*;

        mod success {
            use super::*;

            #[test]
            #[parallel]
            fn it_generates_template_using_provided_client() {
                let template_names = make_string_vec("rust python");
                let generated_template = "all good";
                let generator_url = format!(
                    "{}/rust,python",
                    constant::template_manager::GENERATOR_URI
                );
                let http_client = MockEndpointHttpClient {
                    response: HashMap::from([
                        (generator_url, Ok(String::from(generated_template))),
                        (
                            constant::template_manager::LISTER_URI.to_string(),
                            Ok(String::from("rust\npython")),
                        ),
                    ]),
                };
                let generator = RemoteGitignoreTemplateManager::new(
                    Box::new(http_client),
                    Some(constant::template_manager::GENERATOR_URI),
                    Some(constant::template_manager::LISTER_URI),
                );

                let actual =
                    generator.generate_with_template_check(&template_names);
                let expected: Result<QualifiedString, ProgramExit> =
                    Ok(QualifiedString {
                        value: String::from(generated_template),
                        kind: StringKind::Remote,
                    });

                assert_eq!(actual, expected);
            }

            #[test]
            #[parallel]
            fn it_works_with_none_generator_endpoint_uri() {
                let template_names = make_string_vec("rust python");
                let generated_template = "all good";
                let generator_url = format!(
                    "{}/rust,python",
                    constant::template_manager::GENERATOR_URI
                );
                let http_client = MockEndpointHttpClient {
                    response: HashMap::from([
                        (generator_url, Ok(String::from(generated_template))),
                        (
                            constant::template_manager::LISTER_URI.to_string(),
                            Ok(String::from("rust\npython")),
                        ),
                    ]),
                };
                let generator = RemoteGitignoreTemplateManager::new(
                    Box::new(http_client),
                    None,
                    Some(constant::template_manager::LISTER_URI),
                );

                let actual =
                    generator.generate_with_template_check(&template_names);
                let expected: Result<QualifiedString, ProgramExit> =
                    Ok(QualifiedString {
                        value: String::from(generated_template),
                        kind: StringKind::Remote,
                    });

                assert_eq!(actual, expected);
            }

            #[test]
            #[parallel]
            fn it_works_with_none_lister_endpoint_uri() {
                let template_names = make_string_vec("rust python");
                let generated_template = "all good";
                let generator_url = format!(
                    "{}/rust,python",
                    constant::template_manager::GENERATOR_URI
                );
                let http_client = MockEndpointHttpClient {
                    response: HashMap::from([
                        (generator_url, Ok(String::from(generated_template))),
                        (
                            constant::template_manager::LISTER_URI.to_string(),
                            Ok(String::from("rust\npython")),
                        ),
                    ]),
                };
                let generator = RemoteGitignoreTemplateManager::new(
                    Box::new(http_client),
                    Some(constant::template_manager::GENERATOR_URI),
                    None,
                );

                let actual =
                    generator.generate_with_template_check(&template_names);
                let expected: Result<QualifiedString, ProgramExit> =
                    Ok(QualifiedString {
                        value: String::from(generated_template),
                        kind: StringKind::Remote,
                    });

                assert_eq!(actual, expected);
            }

            #[test]
            #[parallel]
            fn it_works_with_none_lister_and_generator_endpoint_uris() {
                let template_names = make_string_vec("rust python");
                let generated_template = "all good";
                let generator_url = format!(
                    "{}/rust,python",
                    constant::template_manager::GENERATOR_URI
                );
                let http_client = MockEndpointHttpClient {
                    response: HashMap::from([
                        (generator_url, Ok(String::from(generated_template))),
                        (
                            constant::template_manager::LISTER_URI.to_string(),
                            Ok(String::from("rust\npython")),
                        ),
                    ]),
                };
                let generator = RemoteGitignoreTemplateManager::new(
                    Box::new(http_client),
                    None,
                    None,
                );

                let actual =
                    generator.generate_with_template_check(&template_names);
                let expected: Result<QualifiedString, ProgramExit> =
                    Ok(QualifiedString {
                        value: String::from(generated_template),
                        kind: StringKind::Remote,
                    });

                assert_eq!(actual, expected);
            }
        }

        mod failure {
            use super::*;

            #[test]
            #[parallel]
            fn it_propagates_error_from_generator_client_if_any() {
                let template_names = make_string_vec("rust python");
                let error_message = "all bad";
                let generator_url = format!(
                    "{}/rust,python",
                    constant::template_manager::GENERATOR_URI
                );
                let http_client = MockEndpointHttpClient {
                    response: HashMap::from([
                        (
                            generator_url,
                            Err(ProgramExit {
                                message: String::from(error_message),
                                exit_status: constant::exit_status::GENERIC,
                                styled_message: None,
                                kind: ExitKind::Error,
                            }),
                        ),
                        (
                            constant::template_manager::LISTER_URI.to_string(),
                            Ok(String::from("rust\npython")),
                        ),
                    ]),
                };
                let generator = RemoteGitignoreTemplateManager::new(
                    Box::new(http_client),
                    Some(constant::template_manager::GENERATOR_URI),
                    Some(constant::template_manager::LISTER_URI),
                );

                let actual =
                    generator.generate_with_template_check(&template_names);
                let expected: Result<QualifiedString, ProgramExit> =
                    Err(ProgramExit {
                        message: String::from(error_message),
                        exit_status: constant::exit_status::GENERIC,
                        styled_message: None,
                        kind: ExitKind::Error,
                    });

                assert_eq!(actual, expected);
            }

            #[test]
            #[parallel]
            fn it_propagates_error_from_lister_client_if_any() {
                let template_names = make_string_vec("rust python");
                let error_message = "all bad";
                let generated_template = "all good";
                let generator_url = format!(
                    "{}/rust,python",
                    constant::template_manager::GENERATOR_URI
                );
                let http_client = MockEndpointHttpClient {
                    response: HashMap::from([
                        (generator_url, Ok(String::from(generated_template))),
                        (
                            constant::template_manager::LISTER_URI.to_string(),
                            Err(ProgramExit {
                                message: String::from(error_message),
                                exit_status: constant::exit_status::GENERIC,
                                styled_message: None,
                                kind: ExitKind::Error,
                            }),
                        ),
                    ]),
                };
                let generator = RemoteGitignoreTemplateManager::new(
                    Box::new(http_client),
                    Some(constant::template_manager::GENERATOR_URI),
                    Some(constant::template_manager::LISTER_URI),
                );

                let actual =
                    generator.generate_with_template_check(&template_names);
                let expected: Result<QualifiedString, ProgramExit> =
                    Err(ProgramExit {
                        message: String::from(error_message),
                        exit_status: constant::exit_status::GENERIC,
                        styled_message: None,
                        kind: ExitKind::Error,
                    });

                assert_eq!(actual, expected);
            }

            #[test]
            #[parallel]
            fn it_returns_inexistent_template_error() {
                let template_names = make_string_vec("rust pyth");
                let generated_template = "all good";
                let generator_url = format!(
                    "{}/rust,python",
                    constant::template_manager::GENERATOR_URI
                );
                let template_list = "rust\npython";
                let http_client = MockEndpointHttpClient {
                    response: HashMap::from([
                        (generator_url, Ok(String::from(generated_template))),
                        (
                            constant::template_manager::LISTER_URI.to_string(),
                            Ok(String::from(template_list)),
                        ),
                    ]),
                };
                let generator = RemoteGitignoreTemplateManager::new(
                    Box::new(http_client),
                    Some(constant::template_manager::GENERATOR_URI),
                    Some(constant::template_manager::LISTER_URI),
                );

                let actual =
                    generator.generate_with_template_check(&template_names);
                let expected_error_message = String::from(
                    "Following template names are not supported: pyth.\n",
                ) + "For the list of available template names, try "
                    + "'--list'.";
                let expected: Result<QualifiedString, ProgramExit> =
                    Err(ProgramExit {
                        message: expected_error_message,
                        exit_status: constant::exit_status::GENERIC,
                        styled_message: None,
                        kind: ExitKind::Error,
                    });

                assert_eq!(actual, expected);
            }
        }
    }

    mod list {
        use super::*;

        mod success {
            use super::*;

            #[test]
            #[parallel]
            fn it_lists_template_using_provided_client() {
                let template_list = "rust\npython";
                let http_client = MockHttpClient {
                    response: Ok(String::from(template_list)),
                };
                let lister = RemoteGitignoreTemplateManager::new(
                    Box::new(http_client),
                    Some(constant::template_manager::GENERATOR_URI),
                    Some(constant::template_manager::LISTER_URI),
                );

                let actual = lister.list();
                let expected: Result<QualifiedString, ProgramExit> =
                    Ok(QualifiedString {
                        value: String::from(template_list),
                        kind: StringKind::Remote,
                    });

                assert_eq!(actual, expected);
            }

            #[test]
            #[parallel]
            fn it_works_with_none_endpoint_uri() {
                let template_list = "rust\npython";
                let http_client = MockHttpClient {
                    response: Ok(String::from(template_list)),
                };
                let lister = RemoteGitignoreTemplateManager::new(
                    Box::new(http_client),
                    Some(constant::template_manager::GENERATOR_URI),
                    None,
                );

                let actual = lister.list();
                let expected: Result<QualifiedString, ProgramExit> =
                    Ok(QualifiedString {
                        value: String::from(template_list),
                        kind: StringKind::Remote,
                    });

                assert_eq!(actual, expected);
            }
        }

        mod failure {
            use super::*;

            #[test]
            #[parallel]
            fn it_propagates_error_from_client_if_any() {
                let error_message = "all bad";
                let http_client = MockHttpClient {
                    response: Err(ProgramExit {
                        message: String::from(error_message),
                        exit_status: constant::exit_status::GENERIC,
                        styled_message: None,
                        kind: ExitKind::Error,
                    }),
                };
                let lister = RemoteGitignoreTemplateManager::new(
                    Box::new(http_client),
                    Some(constant::template_manager::GENERATOR_URI),
                    Some(constant::template_manager::LISTER_URI),
                );

                let actual = lister.list();
                let expected: Result<QualifiedString, ProgramExit> =
                    Err(ProgramExit {
                        message: String::from(error_message),
                        exit_status: constant::exit_status::GENERIC,
                        styled_message: None,
                        kind: ExitKind::Error,
                    });

                assert_eq!(actual, expected);
            }
        }
    }
}

mod gitignore_template_manager {
    use super::*;

    mod generate {
        use super::*;

        mod success {
            use super::*;

            #[rstest]
            #[serial]
            fn it_generates_template_from_all_provided_managers(
                _ctx: EnvTestContext,
            ) {
                let template_dir = helper::get_resource_path("templates");
                let generator_url = format!(
                    "{}/python",
                    constant::template_manager::GENERATOR_URI
                );
                let http_client = MockEndpointHttpClient {
                    response: HashMap::from([
                        (generator_url, Ok(String::from("all good"))),
                        (
                            constant::template_manager::LISTER_URI.to_string(),
                            Ok(String::from("python")),
                        ),
                    ]),
                };

                let local_generator =
                    LocalGitignoreTemplateManager::new(Some(&template_dir));
                let remote_generator = RemoteGitignoreTemplateManager::new(
                    Box::new(http_client),
                    Some(constant::template_manager::GENERATOR_URI),
                    Some(constant::template_manager::LISTER_URI),
                );
                let manager_list: [&dyn TemplateGenerator; 2] =
                    [&local_generator, &remote_generator];
                let generator = GitignoreTemplateManager::new(&manager_list);

                let expected: Result<QualifiedString, ProgramExit> =
                    Ok(QualifiedString {
                        value: helper::load_expectation_file_as_string(
                            "local_remote_python_rust_template",
                        ),
                        kind: StringKind::Mixed,
                    });
                let actual =
                    generator.generate(&make_string_vec("python rust"));

                assert_eq!(actual, expected);
            }

            #[rstest]
            #[serial]
            fn it_generates_empty_template_when_no_managers(
                _ctx: EnvTestContext,
            ) {
                let generator = GitignoreTemplateManager::new(&[]);

                let expected: Result<QualifiedString, ProgramExit> =
                    Ok(QualifiedString {
                        value: String::new(),
                        kind: StringKind::Mixed,
                    });
                let actual =
                    generator.generate(&make_string_vec("python rust"));

                assert_eq!(actual, expected);
            }
        }

        mod failure {
            use super::*;

            #[rstest]
            #[serial]
            fn it_fails_when_unsupported_template_names_from_all_managers(
                _ctx: EnvTestContext,
            ) {
                let template_dir = helper::get_resource_path("templates");
                let generator_url = format!(
                    "{}/rust",
                    constant::template_manager::GENERATOR_URI
                );
                let http_client = MockEndpointHttpClient {
                    response: HashMap::from([
                        (generator_url, Ok(String::from("all good"))),
                        (
                            constant::template_manager::LISTER_URI.to_string(),
                            Ok(String::from("rust")),
                        ),
                    ]),
                };

                let local_generator =
                    LocalGitignoreTemplateManager::new(Some(&template_dir));
                let remote_lister = RemoteGitignoreTemplateManager::new(
                    Box::new(http_client),
                    Some(constant::template_manager::GENERATOR_URI),
                    Some(constant::template_manager::LISTER_URI),
                );
                let manager_list: [&dyn TemplateGenerator; 2] =
                    [&local_generator, &remote_lister];
                let generator = GitignoreTemplateManager::new(&manager_list);

                let expected: Result<QualifiedString, ProgramExit> =
                    Err(ProgramExit {
                        message: constant::error_messages::UNSUPPORTED_TEMPLATE
                            .to_string(),
                        exit_status: constant::exit_status::GENERIC,
                        styled_message: None,
                        kind: ExitKind::Error,
                    });
                let actual = generator.generate(&make_string_vec("go"));

                assert_eq!(actual, expected);
            }

            #[rstest]
            #[serial]
            fn it_propagates_error_from_remote_manager_if_any(
                _ctx: EnvTestContext,
            ) {
                let template_dir = helper::get_resource_path("templates");
                let http_client = MockHttpClient {
                    response: Err(ProgramExit {
                        message: String::from("all bad"),
                        exit_status: constant::exit_status::GENERIC,
                        styled_message: None,
                        kind: ExitKind::Error,
                    }),
                };

                let local_generator =
                    LocalGitignoreTemplateManager::new(Some(&template_dir));
                let remote_lister = RemoteGitignoreTemplateManager::new(
                    Box::new(http_client),
                    Some(constant::template_manager::GENERATOR_URI),
                    Some(constant::template_manager::LISTER_URI),
                );
                let manager_list: [&dyn TemplateGenerator; 2] =
                    [&local_generator, &remote_lister];
                let generator = GitignoreTemplateManager::new(&manager_list);

                let expected: Result<QualifiedString, ProgramExit> =
                    Err(ProgramExit {
                        message: String::from("all bad"),
                        exit_status: constant::exit_status::GENERIC,
                        styled_message: None,
                        kind: ExitKind::Error,
                    });
                let actual =
                    generator.generate(&make_string_vec("rust python"));

                assert_eq!(actual, expected);
            }

            #[rstest]
            #[serial]
            fn it_propagates_error_from_local_manager_if_any(
                _ctx: EnvTestContext,
            ) {
                let template_dir =
                    helper::get_resource_path("templates/python.txt");
                let http_client = MockHttpClient {
                    response: Ok(String::from("all good")),
                };

                let local_generator =
                    LocalGitignoreTemplateManager::new(Some(&template_dir));
                let remote_lister = RemoteGitignoreTemplateManager::new(
                    Box::new(http_client),
                    Some(constant::template_manager::GENERATOR_URI),
                    Some(constant::template_manager::LISTER_URI),
                );
                let manager_list: [&dyn TemplateGenerator; 2] =
                    [&local_generator, &remote_lister];
                let generator = GitignoreTemplateManager::new(&manager_list);

                let expected: Result<QualifiedString, ProgramExit> = Err(
                    ProgramExit {
                        message: "An error occurred while listing templates from local file system: Not a directory (os error 20)".to_string(),
                        exit_status: constant::exit_status::GENERIC,
                        styled_message: None,
                        kind: ExitKind::Error,
                    },
                );
                let actual =
                    generator.generate(&make_string_vec("rust python"));

                assert_eq!(actual, expected);
            }

            #[rstest]
            #[serial]
            fn it_combines_errors_from_all_managers_if_any(
                _ctx: EnvTestContext,
            ) {
                let template_dir =
                    helper::get_resource_path("templates/python.txt");
                let http_client = MockHttpClient {
                    response: Err(ProgramExit {
                        message: String::from("all bad"),
                        exit_status: constant::exit_status::GENERIC,
                        styled_message: None,
                        kind: ExitKind::Error,
                    }),
                };

                let local_generator =
                    LocalGitignoreTemplateManager::new(Some(&template_dir));
                let remote_lister = RemoteGitignoreTemplateManager::new(
                    Box::new(http_client),
                    Some(constant::template_manager::GENERATOR_URI),
                    Some(constant::template_manager::LISTER_URI),
                );
                let manager_list: [&dyn TemplateGenerator; 2] =
                    [&local_generator, &remote_lister];
                let generator = GitignoreTemplateManager::new(&manager_list);

                let expected: Result<QualifiedString, ProgramExit> = Err(
                    ProgramExit {
                        message: "An error occurred while listing templates from local file system: Not a directory (os error 20)\nall bad".to_string(),
                        exit_status: constant::exit_status::GENERIC * 2,
                        styled_message: None,
                        kind: ExitKind::Error,
                    },
                );
                let actual =
                    generator.generate(&make_string_vec("rust python"));

                assert_eq!(actual, expected);
            }
        }
    }

    mod generate_with_template_check {
        use super::*;

        mod success {
            use super::*;

            #[rstest]
            #[serial]
            fn it_generates_template_from_all_provided_managers(
                _ctx: EnvTestContext,
            ) {
                let template_dir = helper::get_resource_path("templates");
                let generator_url = format!(
                    "{}/python,rust",
                    constant::template_manager::GENERATOR_URI
                );
                let http_client = MockEndpointHttpClient {
                    response: HashMap::from([
                        (generator_url, Ok(String::from("all good"))),
                        (
                            constant::template_manager::LISTER_URI.to_string(),
                            Ok(String::from("python\nrust")),
                        ),
                    ]),
                };

                let local_generator =
                    LocalGitignoreTemplateManager::new(Some(&template_dir));
                let remote_generator = RemoteGitignoreTemplateManager::new(
                    Box::new(http_client),
                    Some(constant::template_manager::GENERATOR_URI),
                    Some(constant::template_manager::LISTER_URI),
                );
                let manager_list: [&dyn TemplateGenerator; 2] =
                    [&local_generator, &remote_generator];
                let generator = GitignoreTemplateManager::new(&manager_list);

                let expected: Result<QualifiedString, ProgramExit> =
                    Ok(QualifiedString {
                        value: helper::load_expectation_file_as_string(
                            "local_remote_python_rust_template",
                        ),
                        kind: StringKind::Mixed,
                    });
                let actual = generator.generate_with_template_check(
                    &make_string_vec("python rust"),
                );

                assert_eq!(actual, expected);
            }
        }

        mod failure {
            use super::*;

            #[rstest]
            #[serial]
            fn it_propagates_error_from_remote_manager_if_any(
                _ctx: EnvTestContext,
            ) {
                let template_dir = helper::get_resource_path("templates");
                let http_client = MockHttpClient {
                    response: Err(ProgramExit {
                        message: String::from("all bad"),
                        exit_status: constant::exit_status::GENERIC,
                        styled_message: None,
                        kind: ExitKind::Error,
                    }),
                };

                let local_generator =
                    LocalGitignoreTemplateManager::new(Some(&template_dir));
                let remote_lister = RemoteGitignoreTemplateManager::new(
                    Box::new(http_client),
                    Some(constant::template_manager::GENERATOR_URI),
                    Some(constant::template_manager::LISTER_URI),
                );
                let manager_list: [&dyn TemplateGenerator; 2] =
                    [&local_generator, &remote_lister];
                let generator = GitignoreTemplateManager::new(&manager_list);

                let expected: Result<QualifiedString, ProgramExit> =
                    Err(ProgramExit {
                        message: String::from("all bad"),
                        exit_status: constant::exit_status::GENERIC,
                        styled_message: None,
                        kind: ExitKind::Error,
                    });
                let actual = generator.generate_with_template_check(
                    &make_string_vec("rust python"),
                );

                assert_eq!(actual, expected);
            }

            #[rstest]
            #[serial]
            fn it_propagates_error_from_local_manager_if_any(
                _ctx: EnvTestContext,
            ) {
                let template_dir = helper::get_resource_path("templates");
                let http_client = MockHttpClient {
                    response: Ok(String::from("all good")),
                };

                let local_generator =
                    LocalGitignoreTemplateManager::new(Some(&template_dir));
                let remote_lister = RemoteGitignoreTemplateManager::new(
                    Box::new(http_client),
                    Some(constant::template_manager::GENERATOR_URI),
                    Some(constant::template_manager::LISTER_URI),
                );
                let manager_list: [&dyn TemplateGenerator; 2] =
                    [&local_generator, &remote_lister];
                let generator = GitignoreTemplateManager::new(&manager_list);

                let expected: Result<QualifiedString, ProgramExit> = Err(
                    ProgramExit {
                        message: "Following template names are not supported: pyth.\nFor the list of available template names, try '--list'.".to_string(),
                        exit_status: constant::exit_status::GENERIC,
                        styled_message: None,
                        kind: ExitKind::Error,
                    },
                );
                let actual = generator.generate_with_template_check(
                    &make_string_vec("rust pyth"),
                );

                assert_eq!(actual, expected);
            }

            #[rstest]
            #[serial]
            fn it_combines_errors_from_all_managers_if_any(
                _ctx: EnvTestContext,
            ) {
                let template_dir =
                    helper::get_resource_path("templates/python.txt");
                let http_client = MockHttpClient {
                    response: Err(ProgramExit {
                        message: String::from("all bad"),
                        exit_status: constant::exit_status::GENERIC,
                        styled_message: None,
                        kind: ExitKind::Error,
                    }),
                };

                let local_generator =
                    LocalGitignoreTemplateManager::new(Some(&template_dir));
                let remote_lister = RemoteGitignoreTemplateManager::new(
                    Box::new(http_client),
                    Some(constant::template_manager::GENERATOR_URI),
                    Some(constant::template_manager::LISTER_URI),
                );
                let manager_list: [&dyn TemplateGenerator; 2] =
                    [&local_generator, &remote_lister];
                let generator = GitignoreTemplateManager::new(&manager_list);

                let expected: Result<QualifiedString, ProgramExit> =
                    Err(ProgramExit {
                        message: format!(
                            "{}: Not a directory (os error 20)\nall bad",
                            constant::error_messages::LOCAL_LISTING
                        ),
                        exit_status: constant::exit_status::GENERIC * 2,
                        styled_message: None,
                        kind: ExitKind::Error,
                    });
                let actual = generator.generate_with_template_check(
                    &make_string_vec("rust python"),
                );

                assert_eq!(actual, expected);
            }
        }
    }

    mod list {
        use super::*;

        mod success {
            use super::*;

            #[rstest]
            #[serial]
            fn it_lists_templates_from_all_provided_managers(
                _ctx: EnvTestContext,
            ) {
                let template_dir = helper::get_resource_path("templates");
                let http_client = MockHttpClient {
                    response: Ok(String::from("go\ncpp\nwo")),
                };

                let local_lister =
                    LocalGitignoreTemplateManager::new(Some(&template_dir));
                let remote_lister = RemoteGitignoreTemplateManager::new(
                    Box::new(http_client),
                    Some(constant::template_manager::GENERATOR_URI),
                    Some(constant::template_manager::LISTER_URI),
                );
                let manager_list: [&dyn TemplateGenerator; 2] =
                    [&local_lister, &remote_lister];
                let lister = GitignoreTemplateManager::new(&manager_list);

                let expected: Result<QualifiedString, ProgramExit> =
                    Ok(QualifiedString {
                        value: "cpp\ngo\n*python\n*rust\nwo".to_string(),
                        kind: StringKind::Mixed,
                    });
                let actual = lister.list();

                assert_eq!(actual, expected);
            }
        }

        mod failure {
            use super::*;

            #[rstest]
            #[serial]
            fn it_propagates_error_from_remote_manager_if_any(
                _ctx: EnvTestContext,
            ) {
                let template_dir = helper::get_resource_path("templates");
                let http_client = MockHttpClient {
                    response: Err(ProgramExit {
                        message: String::from("all bad"),
                        exit_status: constant::exit_status::GENERIC,
                        styled_message: None,
                        kind: ExitKind::Error,
                    }),
                };

                let local_lister =
                    LocalGitignoreTemplateManager::new(Some(&template_dir));
                let remote_lister = RemoteGitignoreTemplateManager::new(
                    Box::new(http_client),
                    Some(constant::template_manager::GENERATOR_URI),
                    Some(constant::template_manager::LISTER_URI),
                );
                let manager_list: [&dyn TemplateGenerator; 2] =
                    [&local_lister, &remote_lister];
                let lister = GitignoreTemplateManager::new(&manager_list);

                let expected: Result<QualifiedString, ProgramExit> =
                    Err(ProgramExit {
                        message: String::from("all bad"),
                        exit_status: constant::exit_status::GENERIC,
                        styled_message: None,
                        kind: ExitKind::Error,
                    });
                let actual = lister.list();

                assert_eq!(actual, expected);
            }

            #[rstest]
            #[serial]
            fn it_propagates_error_from_local_manager_if_any(
                _ctx: EnvTestContext,
            ) {
                let template_dir =
                    helper::get_resource_path("templates/python.txt");
                let http_client = MockHttpClient {
                    response: Ok(String::from("go\ncpp")),
                };

                let local_lister =
                    LocalGitignoreTemplateManager::new(Some(&template_dir));
                let remote_lister = RemoteGitignoreTemplateManager::new(
                    Box::new(http_client),
                    Some(constant::template_manager::GENERATOR_URI),
                    Some(constant::template_manager::LISTER_URI),
                );
                let manager_list: [&dyn TemplateGenerator; 2] =
                    [&local_lister, &remote_lister];
                let lister = GitignoreTemplateManager::new(&manager_list);

                let expected: Result<QualifiedString, ProgramExit> =
                    Err(ProgramExit {
                        message: format!(
                            "{}: Not a directory (os error 20)",
                            constant::error_messages::LOCAL_LISTING
                        ),
                        exit_status: constant::exit_status::GENERIC,
                        styled_message: None,
                        kind: ExitKind::Error,
                    });
                let actual = lister.list();

                assert_eq!(actual, expected);
            }

            #[rstest]
            #[serial]
            fn it_combines_errors_from_all_managers_if_any(
                _ctx: EnvTestContext,
            ) {
                let template_dir =
                    helper::get_resource_path("templates/python.txt");
                let http_client = MockHttpClient {
                    response: Err(ProgramExit {
                        message: String::from("all bad"),
                        exit_status: constant::exit_status::GENERIC,
                        styled_message: None,
                        kind: ExitKind::Error,
                    }),
                };

                let local_lister =
                    LocalGitignoreTemplateManager::new(Some(&template_dir));
                let remote_lister = RemoteGitignoreTemplateManager::new(
                    Box::new(http_client),
                    Some(constant::template_manager::GENERATOR_URI),
                    Some(constant::template_manager::LISTER_URI),
                );
                let manager_list: [&dyn TemplateGenerator; 2] =
                    [&local_lister, &remote_lister];
                let lister = GitignoreTemplateManager::new(&manager_list);

                let expected: Result<QualifiedString, ProgramExit> =
                    Err(ProgramExit {
                        message: format!(
                            "{}: Not a directory (os error 20)\nall bad",
                            constant::error_messages::LOCAL_LISTING
                        ),
                        exit_status: constant::exit_status::GENERIC * 2,
                        styled_message: None,
                        kind: ExitKind::Error,
                    });
                let actual = lister.list();

                assert_eq!(actual, expected);
            }
        }
    }
}
