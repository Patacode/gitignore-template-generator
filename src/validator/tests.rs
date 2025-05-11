use super::*;
use crate::constant;

mod default_cli_args_validator {
    use super::*;

    mod has_no_commas {
        use super::*;

        mod success {
            use super::*;

            #[test]
            fn it_returns_ok_for_comma_free_values() {
                let value = "rust";

                let expected: Result<String, String> = Ok(String::from(value));
                let actual = DefaultCliArgsValidator::has_no_commas(value);

                assert_eq!(actual, expected);
            }
        }

        mod failure {
            use super::*;

            #[test]
            fn it_returns_error_for_values_with_commas() {
                let value = "r,ust";

                let expected: Result<String, String> = Err(String::from(
                    constant::error_messages::COMMAS_NOT_ALLOWED,
                ));
                let actual = DefaultCliArgsValidator::has_no_commas(value);

                assert_eq!(actual, expected);
            }
        }
    }

    mod has_no_whitespaces {
        use super::*;

        mod success {
            use super::*;

            #[test]
            fn it_returns_ok_for_whitespace_free_values() {
                let value = "rust";

                let expected: Result<String, String> = Ok(String::from(value));
                let actual = DefaultCliArgsValidator::has_no_whitespaces(value);

                assert_eq!(actual, expected);
            }
        }

        mod failure {
            use super::*;

            #[test]
            fn it_returns_error_for_values_with_whitespaces() {
                let value = "r ust";

                let expected: Result<String, String> = Err(String::from(
                    constant::error_messages::WHITESPACES_NOT_ALLOWED,
                ));
                let actual = DefaultCliArgsValidator::has_no_whitespaces(value);

                assert_eq!(actual, expected);
            }
        }
    }

    mod has_valid_template_name {
        use super::*;

        mod success {
            use super::*;

            #[test]
            fn it_returns_ok_for_comma_and_whitespace_free_values() {
                let value = "rust";

                let expected: Result<String, String> = Ok(String::from(value));
                let actual =
                    DefaultCliArgsValidator::has_valid_template_name(value);

                assert_eq!(actual, expected);
            }
        }

        mod failure {
            use super::*;

            #[test]
            fn it_returns_error_for_values_with_whitespaces() {
                let value = "r ust";

                let expected: Result<String, String> = Err(String::from(
                    constant::error_messages::WHITESPACES_NOT_ALLOWED,
                ));
                let actual =
                    DefaultCliArgsValidator::has_valid_template_name(value);

                assert_eq!(actual, expected);
            }

            #[test]
            fn it_returns_error_for_values_with_commas() {
                let value = "r,ust";

                let expected: Result<String, String> = Err(String::from(
                    constant::error_messages::COMMAS_NOT_ALLOWED,
                ));
                let actual =
                    DefaultCliArgsValidator::has_valid_template_name(value);

                assert_eq!(actual, expected);
            }

            #[test]
            fn it_returns_error_for_values_with_commas_and_whitespaces() {
                let value = "r, ust";

                let expected: Result<String, String> = Err(format!(
                    "{} + {}",
                    constant::error_messages::COMMAS_NOT_ALLOWED,
                    constant::error_messages::WHITESPACES_NOT_ALLOWED,
                ));
                let actual =
                    DefaultCliArgsValidator::has_valid_template_name(value);

                assert_eq!(actual, expected);
            }
        }
    }
}
