pub use crate::core::impls::GitignoreTemplateGenerator;
use crate::http_client::{HttpClient, ProgramError};

pub trait TemplateGenerator {
    fn generate_from_api(
        http_client: &dyn HttpClient,
        template_names: &str,
    ) -> Result<String, ProgramError>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::http_client::MockClient;

    mod gitignore_template_generator {
        use super::*;

        mod generate_from_api {
            use super::*;

            mod success {
                use super::*;

                #[test]
                fn it_generates_template_using_provided_client() {
                    let template_names = "rust,python";
                    let generated_template = "all good";
                    let http_client = MockClient {
                        response: Ok(String::from(generated_template)),
                    };

                    let actual = GitignoreTemplateGenerator::generate_from_api(
                        &http_client,
                        template_names,
                    );
                    let expected: Result<String, ProgramError> =
                        Ok(String::from(generated_template));

                    assert_eq!(actual, expected);
                }
            }

            mod failure {
                use crate::constant;

                use super::*;

                #[test]
                fn it_propagates_error_from_client_if_any() {
                    let template_names = "rust,pyth";
                    let error_message = "all bad";
                    let http_client = MockClient {
                        response: Err(ProgramError {
                            message: String::from(error_message),
                            exit_status: constant::exit_status::GENERIC,
                        }),
                    };

                    let actual = GitignoreTemplateGenerator::generate_from_api(
                        &http_client,
                        template_names,
                    );
                    let expected: Result<String, ProgramError> =
                        Err(ProgramError {
                            message: String::from(error_message),
                            exit_status: constant::exit_status::GENERIC,
                        });

                    assert_eq!(actual, expected);
                }
            }
        }
    }
}
