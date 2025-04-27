//! Define components used to wrap core logic.
//!
//! As per crate definition, core logic is defined as generating
//! gitignore templates via a template generator API over HTTP. So you will
//! find methods to respond to that need.

pub use crate::core::impls::GitignoreTemplateGenerator;
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
pub trait TemplateGenerator {
    /// Generates a string template matching given template names.
    ///
    /// The template is generated via a GET API call made over HTTP using
    /// the given http client.
    ///
    /// # Arguments
    /// * `http_client` - The http client to be used to make the API call.
    /// * `template_names` - The template names to be used to generated the
    ///     actual template
    ///
    /// # Returns
    ///
    /// A result containing the generated template on success, or a
    /// [`ProgramExit`] on error (e.g. 4xx, network issues...).
    fn generate_from_api(
        http_client: &impl HttpClient,
        template_names: &[String],
    ) -> Result<String, ProgramExit>;
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
                    let expected: Result<String, ProgramExit> =
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
                        response: Err(ProgramExit {
                            message: String::from(error_message),
                            exit_status: constant::exit_status::GENERIC,
                            styled_message: None,
                            kind: ExitKind::Error,
                        }),
                    };

                    let actual = GitignoreTemplateGenerator::generate_from_api(
                        &http_client,
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
    }
}
