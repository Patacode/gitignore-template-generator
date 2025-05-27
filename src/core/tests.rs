use std::collections::HashMap;

use serial_test::serial;

use super::*;
use crate::{
    constant,
    helper::{self, make_string_vec},
    http_client::{MockEndpointHttpClient, MockHttpClient},
};

mod gitignore_template_generator {
    use super::*;

    mod generate_locally {
        use super::*;

        mod success {
            use super::*;

            #[test]
            #[serial]
            fn it_generates_template_from_local_fs_using_env_var() {
                let template_dir = helper::get_resource_path("templates");
                let template_names = make_string_vec("python rust");
                unsafe {
                    std::env::set_var(
                        constant::template_manager::HOME_ENV_VAR,
                        &template_dir,
                    );
                }

                let expected_template = helper::load_expectation_file_as_string(
                    "local_rust_python_template",
                );
                let actual_template =
                    GitignoreTemplateManager::generate_locally(
                        &template_dir,
                        &template_names,
                    );

                assert!(actual_template.is_ok());

                let actual_template = actual_template.unwrap();
                assert_eq!(actual_template, expected_template);

                unsafe {
                    std::env::remove_var(
                        constant::template_manager::HOME_ENV_VAR,
                    );
                }
            }

            #[test]
            fn it_generates_template_from_local_fs_using_given_dir() {
                let template_dir = helper::get_resource_path("templates");
                let template_names = make_string_vec("python rust");

                let expected_template = helper::load_expectation_file_as_string(
                    "local_rust_python_template",
                );
                let actual_template =
                    GitignoreTemplateManager::generate_locally(
                        &template_dir,
                        &template_names,
                    );

                assert!(actual_template.is_ok());

                let actual_template = actual_template.unwrap();
                assert_eq!(actual_template, expected_template);
            }
        }

        mod failure {
            use super::*;

            #[test]
            fn it_fails_when_unsupported_template_names() {
                let template_dir = helper::get_resource_path("templates");
                let template_names = make_string_vec("python rust unknown");

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
                let actual_error = GitignoreTemplateManager::generate_locally(
                    &template_dir,
                    &template_names,
                );

                assert!(actual_error.is_err());

                let actual_error = actual_error.unwrap_err();
                assert_eq!(actual_error, expected_error);
            }

            #[test]
            fn it_propagates_fs_error_if_any_occurred() {
                let template_dir = helper::get_resource_path("templates");
                let template_names = make_string_vec("dummy");

                let expected_error = ProgramExit {
                    message: format!(
                        "{}: Is a directory (os error 21)",
                        constant::error_messages::LOCAL_GENERATION
                    ),
                    exit_status: constant::exit_status::GENERIC,
                    styled_message: None,
                    kind: ExitKind::Error,
                };
                let actual_error = GitignoreTemplateManager::generate_locally(
                    &template_dir,
                    &template_names,
                );

                assert!(actual_error.is_err());

                let actual_error = actual_error.unwrap_err();
                assert_eq!(actual_error, expected_error);
            }
        }
    }

    mod generate_from_api {
        use super::*;

        mod success {
            use super::*;

            #[test]
            fn it_generates_template_using_provided_client() {
                let template_names = make_string_vec("rust python");
                let generated_template = "all good";
                let http_client = MockHttpClient {
                    response: Ok(String::from(generated_template)),
                };

                let actual = GitignoreTemplateManager::generate_from_api(
                    &http_client,
                    Some(constant::template_manager::GENERATOR_URI),
                    &template_names,
                );
                let expected: Result<String, ProgramExit> =
                    Ok(String::from(generated_template));

                assert_eq!(actual, expected);
            }

            #[test]
            fn it_works_with_none_endpoint_uri() {
                let template_names = make_string_vec("rust python");
                let generated_template = "all good";
                let http_client = MockHttpClient {
                    response: Ok(String::from(generated_template)),
                };

                let actual = GitignoreTemplateManager::generate_from_api(
                    &http_client,
                    None,
                    &template_names,
                );
                let expected: Result<String, ProgramExit> =
                    Ok(String::from(generated_template));

                assert_eq!(actual, expected);
            }
        }

        mod failure {
            use super::*;

            #[test]
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

                let actual = GitignoreTemplateManager::generate_from_api(
                    &http_client,
                    Some(constant::template_manager::GENERATOR_URI),
                    &template_names,
                );
                let expected: Result<String, ProgramExit> = Err(ProgramExit {
                    message: String::from(error_message),
                    exit_status: constant::exit_status::GENERIC,
                    styled_message: None,
                    kind: ExitKind::Error,
                });

                assert_eq!(actual, expected);
            }
        }
    }

    mod generate_from_api_with_template_check {
        use super::*;

        mod success {
            use super::*;

            #[test]
            fn it_generates_template_using_provided_client() {
                let template_names = make_string_vec("rust python");
                let generated_template = "all good";
                let generator_url = format!(
                    "{}/rust,python",
                    constant::template_manager::GENERATOR_URI
                );
                let http_client = MockEndpointHttpClient {
                    response: HashMap::from([
                        (
                            generator_url.as_str(),
                            Ok(String::from(generated_template)),
                        ),
                        (
                            constant::template_manager::LISTER_URI,
                            Ok(String::from("rust\npython")),
                        ),
                    ]),
                };

                let actual = GitignoreTemplateManager::generate_from_api_with_template_check(
                    &http_client,
                    Some(constant::template_manager::GENERATOR_URI),
                    Some(constant::template_manager::LISTER_URI),
                    &template_names,
                );
                let expected: Result<String, ProgramExit> =
                    Ok(String::from(generated_template));

                assert_eq!(actual, expected);
            }

            #[test]
            fn it_works_with_none_generator_endpoint_uri() {
                let template_names = make_string_vec("rust python");
                let generated_template = "all good";
                let generator_url = format!(
                    "{}/rust,python",
                    constant::template_manager::GENERATOR_URI
                );
                let http_client = MockEndpointHttpClient {
                    response: HashMap::from([
                        (
                            generator_url.as_str(),
                            Ok(String::from(generated_template)),
                        ),
                        (
                            constant::template_manager::LISTER_URI,
                            Ok(String::from("rust\npython")),
                        ),
                    ]),
                };

                let actual = GitignoreTemplateManager::generate_from_api_with_template_check(
                    &http_client,
                    None,
                    Some(constant::template_manager::LISTER_URI),
                    &template_names,
                );
                let expected: Result<String, ProgramExit> =
                    Ok(String::from(generated_template));

                assert_eq!(actual, expected);
            }

            #[test]
            fn it_works_with_none_lister_endpoint_uri() {
                let template_names = make_string_vec("rust python");
                let generated_template = "all good";
                let generator_url = format!(
                    "{}/rust,python",
                    constant::template_manager::GENERATOR_URI
                );
                let http_client = MockEndpointHttpClient {
                    response: HashMap::from([
                        (
                            generator_url.as_str(),
                            Ok(String::from(generated_template)),
                        ),
                        (
                            constant::template_manager::LISTER_URI,
                            Ok(String::from("rust\npython")),
                        ),
                    ]),
                };

                let actual = GitignoreTemplateManager::generate_from_api_with_template_check(
                    &http_client,
                    Some(constant::template_manager::GENERATOR_URI),
                    None,
                    &template_names,
                );
                let expected: Result<String, ProgramExit> =
                    Ok(String::from(generated_template));

                assert_eq!(actual, expected);
            }

            #[test]
            fn it_works_with_none_lister_and_generator_endpoint_uris() {
                let template_names = make_string_vec("rust python");
                let generated_template = "all good";
                let generator_url = format!(
                    "{}/rust,python",
                    constant::template_manager::GENERATOR_URI
                );
                let http_client = MockEndpointHttpClient {
                    response: HashMap::from([
                        (
                            generator_url.as_str(),
                            Ok(String::from(generated_template)),
                        ),
                        (
                            constant::template_manager::LISTER_URI,
                            Ok(String::from("rust\npython")),
                        ),
                    ]),
                };

                let actual = GitignoreTemplateManager::generate_from_api_with_template_check(
                    &http_client,
                    None,
                    None,
                    &template_names,
                );
                let expected: Result<String, ProgramExit> =
                    Ok(String::from(generated_template));

                assert_eq!(actual, expected);
            }
        }

        mod failure {
            use super::*;

            #[test]
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
                            generator_url.as_str(),
                            Err(ProgramExit {
                                message: String::from(error_message),
                                exit_status: constant::exit_status::GENERIC,
                                styled_message: None,
                                kind: ExitKind::Error,
                            }),
                        ),
                        (
                            constant::template_manager::LISTER_URI,
                            Ok(String::from("rust\npython")),
                        ),
                    ]),
                };

                let actual = GitignoreTemplateManager::generate_from_api_with_template_check(
                    &http_client,
                    Some(constant::template_manager::GENERATOR_URI),
                    Some(constant::template_manager::LISTER_URI),
                    &template_names,
                );
                let expected: Result<String, ProgramExit> = Err(ProgramExit {
                    message: String::from(error_message),
                    exit_status: constant::exit_status::GENERIC,
                    styled_message: None,
                    kind: ExitKind::Error,
                });

                assert_eq!(actual, expected);
            }

            #[test]
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
                        (
                            generator_url.as_str(),
                            Ok(String::from(generated_template)),
                        ),
                        (
                            constant::template_manager::LISTER_URI,
                            Err(ProgramExit {
                                message: String::from(error_message),
                                exit_status: constant::exit_status::GENERIC,
                                styled_message: None,
                                kind: ExitKind::Error,
                            }),
                        ),
                    ]),
                };

                let actual = GitignoreTemplateManager::generate_from_api_with_template_check(
                    &http_client,
                    Some(constant::template_manager::GENERATOR_URI),
                    Some(constant::template_manager::LISTER_URI),
                    &template_names,
                );
                let expected: Result<String, ProgramExit> = Err(ProgramExit {
                    message: String::from(error_message),
                    exit_status: constant::exit_status::GENERIC,
                    styled_message: None,
                    kind: ExitKind::Error,
                });

                assert_eq!(actual, expected);
            }

            #[test]
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
                        (
                            generator_url.as_str(),
                            Ok(String::from(generated_template)),
                        ),
                        (
                            constant::template_manager::LISTER_URI,
                            Ok(String::from(template_list)),
                        ),
                    ]),
                };

                let actual = GitignoreTemplateManager::generate_from_api_with_template_check(
                    &http_client,
                    Some(constant::template_manager::GENERATOR_URI),
                    Some(constant::template_manager::LISTER_URI),
                    &template_names,
                );
                let expected_error_message = String::from(
                    "Following template names are not supported: pyth.\n",
                ) + "For the list of available template names, try "
                    + "'--list'.";
                let expected: Result<String, ProgramExit> = Err(ProgramExit {
                    message: expected_error_message,
                    exit_status: constant::exit_status::GENERIC,
                    styled_message: None,
                    kind: ExitKind::Error,
                });

                assert_eq!(actual, expected);
            }
        }
    }

    mod list_locally {
        use super::*;

        mod success {
            use super::*;

            #[test]
            #[serial]
            fn it_lists_templates_from_local_fs_using_env_var() {
                let template_dir = helper::get_resource_path("templates");
                unsafe {
                    std::env::set_var(
                        constant::template_manager::HOME_ENV_VAR,
                        &template_dir,
                    );
                }

                let expected_list = "*python\n*rust";
                let actual_list =
                    GitignoreTemplateManager::list_locally(&template_dir);

                assert!(actual_list.is_ok());

                let actual_list = actual_list.unwrap();
                assert_eq!(actual_list, expected_list);

                unsafe {
                    std::env::remove_var(
                        constant::template_manager::HOME_ENV_VAR,
                    );
                }
            }

            #[test]
            fn it_lists_templates_from_local_fs_using_given_dir() {
                let template_dir = helper::get_resource_path("templates");

                let expected_list = "*python\n*rust";
                let actual_list =
                    GitignoreTemplateManager::list_locally(&template_dir);

                assert!(actual_list.is_ok());

                let actual_list = actual_list.unwrap();
                assert_eq!(actual_list, expected_list);
            }

            #[test]
            fn it_returns_empty_string_when_inexistent_dir() {
                let template_dir = helper::get_resource_path("inexistent");

                let expected_list = "";
                let actual_list =
                    GitignoreTemplateManager::list_locally(&template_dir);

                assert!(actual_list.is_ok());

                let actual_list = actual_list.unwrap();
                assert_eq!(actual_list, expected_list);
            }
        }

        mod failure {
            use super::*;

            #[test]
            fn it_propagates_fs_error_if_any_occurred() {
                let template_dir =
                    helper::get_resource_path("templates/python.txt");

                let expected_error = ProgramExit {
                    message: format!(
                        "{}: Not a directory (os error 20)",
                        constant::error_messages::LOCAL_LISTING
                    ),
                    exit_status: constant::exit_status::GENERIC,
                    styled_message: None,
                    kind: ExitKind::Error,
                };
                let actual_error =
                    GitignoreTemplateManager::list_locally(&template_dir);

                assert!(actual_error.is_err());

                let actual_error = actual_error.unwrap_err();
                assert_eq!(actual_error, expected_error);
            }
        }
    }

    mod list_from_api {
        use super::*;

        mod success {
            use super::*;

            #[test]
            fn it_lists_template_using_provided_client() {
                let template_list = "rust\npython";
                let http_client = MockHttpClient {
                    response: Ok(String::from(template_list)),
                };

                let actual = GitignoreTemplateManager::list_from_api(
                    &http_client,
                    Some(constant::template_manager::LISTER_URI),
                );
                let expected: Result<String, ProgramExit> =
                    Ok(String::from(template_list));

                assert_eq!(actual, expected);
            }

            #[test]
            fn it_works_with_none_endpoint_uri() {
                let template_list = "rust\npython";
                let http_client = MockHttpClient {
                    response: Ok(String::from(template_list)),
                };

                let actual =
                    GitignoreTemplateManager::list_from_api(&http_client, None);
                let expected: Result<String, ProgramExit> =
                    Ok(String::from(template_list));

                assert_eq!(actual, expected);
            }
        }

        mod failure {
            use super::*;

            #[test]
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

                let actual = GitignoreTemplateManager::list_from_api(
                    &http_client,
                    Some(constant::template_manager::LISTER_URI),
                );
                let expected: Result<String, ProgramExit> = Err(ProgramExit {
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
