pub use crate::core::impls::GitignoreTemplateGenerator;
use crate::http_client::HttpClient;

#[derive(Clone, PartialEq, Debug)]
pub struct ProgramError {
    pub message: String,
    pub exit_status: i32,
    pub styled_message: Option<String>,
    pub error_kind: Option<ErrorKind>,
}

#[derive(Clone, PartialEq, Debug)]
pub enum ErrorKind {
    VersionInfos,
    HelpInfos,
    AuthorInfos,
    Other,
}

pub trait TemplateGenerator {
    fn generate_from_api(
        http_client: &dyn HttpClient,
        template_names: &Vec<String>,
    ) -> Result<String, ProgramError>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::helper::make_string_vec;
    use crate::http_client::MockClient;

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
                    let http_client = MockClient {
                        response: Ok(String::from(generated_template)),
                    };

                    let actual = GitignoreTemplateGenerator::generate_from_api(
                        &http_client,
                        &template_names,
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
                    let template_names = make_string_vec("rust pyth");
                    let error_message = "all bad";
                    let http_client = MockClient {
                        response: Err(ProgramError {
                            message: String::from(error_message),
                            exit_status: constant::exit_status::GENERIC,
                            styled_message: None,
                            error_kind: None,
                        }),
                    };

                    let actual = GitignoreTemplateGenerator::generate_from_api(
                        &http_client,
                        &template_names,
                    );
                    let expected: Result<String, ProgramError> =
                        Err(ProgramError {
                            message: String::from(error_message),
                            exit_status: constant::exit_status::GENERIC,
                            styled_message: None,
                            error_kind: None,
                        });

                    assert_eq!(actual, expected);
                }
            }
        }
    }
}
