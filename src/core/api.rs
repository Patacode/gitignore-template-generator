pub use crate::core::impls::GitignoreTemplateGenerator;
use crate::http_client::{HttpClient, ProgramError};

pub trait TemplateGenerator {
    fn generate_from_api(
        http_client: &dyn HttpClient,
        values: &str,
    ) -> Result<String, ProgramError>;
}

#[cfg(test)]
mod tests {
    use crate::http_client::MockClient;

    use super::*;

    #[test]
    fn it_generates_template_from_api_call_with_gitignore_generator() {
        let client = MockClient {
            response: Ok(String::from("all good")),
        };
        let values = "rust,python";

        let expected: Result<String, ProgramError> =
            Ok(String::from("all good"));
        let actual =
            GitignoreTemplateGenerator::generate_from_api(&client, values);

        assert_eq!(actual, expected);
    }

    #[test]
    fn it_propagates_error_if_any_with_gitignore_generator() {
        let client = MockClient {
            response: Err(ProgramError {
                message: String::from("all bad"),
                exit_status: 2,
            }),
        };
        let values = "rust,python";

        let expected: Result<String, ProgramError> = Err(ProgramError {
            message: String::from("all bad"),
            exit_status: 2,
        });
        let actual =
            GitignoreTemplateGenerator::generate_from_api(&client, values);

        assert_eq!(actual, expected);
    }
}
