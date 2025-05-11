pub use crate::core::impls::GitignoreTemplateManager;
use crate::http_client::HttpClient;

/// DTO struct representing an early or abrupt program exit.
#[derive(Clone, PartialEq, Debug)]
pub struct ProgramExit {
    /// The message linked to the program exit.
    pub message: String,

    /// The exit status code to be returned by the script.
    pub exit_status: i32,

    /// The ANSI-styled message linked to the program exit.
    ///
    /// Same as [`ProgramExit::message`] but styled.
    pub styled_message: Option<String>,

    /// The kind of program exit.
    pub kind: ExitKind,
}

/// Enum for kind of program exit.
#[derive(Clone, PartialEq, Debug)]
pub enum ExitKind {
    /// Early program exit to print version infos.
    VersionInfos,

    /// Early program exit to print help infos.
    HelpInfos,

    /// Early program exit to print author infos.
    AuthorInfos,

    /// Abrupt program exit due to runtime error.
    Error,
}

/// Template generator trait to generate a template via an API call made
/// over HTTP.
pub trait TemplateGenerator: TemplateLister {
    /// Generates a string template matching given template names.
    ///
    /// The template is generated via a GET API call made over HTTP using
    /// the given http client.
    ///
    /// # Arguments
    ///
    /// * `http_client` - The http client to be used to make the API call.
    /// * `endpoint_uri` - The endpoint URI to generate templates (defaults to
    ///     [`crate::constant::template_manager::GENERATOR_URI`] if None).
    /// * `template_names` - The template names to be used to generated the
    ///     actual template.
    ///
    /// # Returns
    ///
    /// A result containing the generated template on success, or a
    /// [`ProgramExit`] on error (e.g. 4xx, network issues...).
    fn generate_from_api(
        http_client: &impl HttpClient,
        endpoint_uri: Option<&str>,
        template_names: &[String],
    ) -> Result<String, ProgramExit>;

    /// Generates a string template matching given template names with robust
    /// template checks.
    ///
    /// Behaves the same as [`TemplateGenerator::generate_from_api`] but will
    /// return a detailed error message in case any of provided template
    /// names are not listed in available template list (as returned by
    /// [`TemplateLister::list_from_api`]).
    ///
    /// # Arguments
    ///
    /// * `http_client` - The http client to be used to make the API call.
    /// * `generator_endpoint_uri` - The endpoint URI to generate templates
    ///     (defaults to [`crate::constant::template_manager::GENERATOR_URI`]
    ///     if None).
    /// * `endpoint_uri` - The endpoint URI to list templates (defaults to
    ///     [`crate::constant::template_manager::LISTER_URI`] if None).
    /// * `template_names` - The template names to be used to generated the
    ///     actual template.
    ///
    /// # Returns
    ///
    /// A result containing the generated template on success, or a
    /// [`ProgramExit`] on error (e.g. 4xx, network issues, template not
    /// found...).
    fn generate_from_api_with_template_check(
        http_client: &impl HttpClient,
        generator_endpoint_uri: Option<&str>,
        lister_endpoint_uri: Option<&str>,
        template_names: &[String],
    ) -> Result<String, ProgramExit>;
}

/// Template lister trait to list available templates via an API call made
/// over HTTP.
pub trait TemplateLister {
    /// Lists available templates.
    ///
    /// The template list is fetched via a GET API call made over HTTP using
    /// the given http client.
    ///
    /// # Arguments
    ///
    /// * `http_client` - The http client to be used to make the API call.
    /// * `endpoint_uri` - The endpoint URI to list templates (defaults to
    ///     [`crate::constant::template_manager::LISTER_URI`] if None).
    ///
    /// # Returns
    ///
    /// A result containing the template list on success, or a
    /// [`ProgramExit`] on error (e.g. 4xx, network issues...).
    fn list_from_api(
        http_client: &impl HttpClient,
        endpoint_uri: Option<&str>,
    ) -> Result<String, ProgramExit>;
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;
    use crate::{
        constant,
        helper::make_string_vec,
        http_client::{MockEndpointHttpClient, MockHttpClient},
    };

    mod gitignore_template_generator {
        use super::*;

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
                    let expected: Result<String, ProgramExit> =
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
                    let expected: Result<String, ProgramExit> =
                        Err(ProgramExit {
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
                    let expected: Result<String, ProgramExit> =
                        Err(ProgramExit {
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
                        "Following template names are not supported: pyth. ",
                    ) + "For the list of available template names, try "
                        + "'--list'.";
                    let expected: Result<String, ProgramExit> =
                        Err(ProgramExit {
                            message: String::from(expected_error_message),
                            exit_status: constant::exit_status::GENERIC,
                            styled_message: None,
                            kind: ExitKind::Error,
                        });

                    assert_eq!(actual, expected);
                }

                #[test]
                fn it_returns_inexistent_templates_error() {
                    let template_names = make_string_vec("rust pyth foo");
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
                        "Following template names are not supported: pyth, ",
                    ) + "foo. For the list of available template names, try "
                        + "'--list'.";
                    let expected: Result<String, ProgramExit> =
                        Err(ProgramExit {
                            message: String::from(expected_error_message),
                            exit_status: constant::exit_status::GENERIC,
                            styled_message: None,
                            kind: ExitKind::Error,
                        });

                    assert_eq!(actual, expected);
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

                    let actual = GitignoreTemplateManager::list_from_api(
                        &http_client,
                        None,
                    );
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
                    let expected: Result<String, ProgramExit> =
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
}
