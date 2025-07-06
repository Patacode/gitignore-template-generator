use rstest::*;

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

                let expected: Result<String, String> =
                    Err(String::from(constant::error_messages::COMMAS_NOT_ALLOWED));
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
                let actual = DefaultCliArgsValidator::is_valid_template_name(value);

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
                let actual = DefaultCliArgsValidator::is_valid_template_name(value);

                assert_eq!(actual, expected);
            }

            #[test]
            fn it_returns_error_for_values_with_commas() {
                let value = "r,ust";

                let expected: Result<String, String> =
                    Err(String::from(constant::error_messages::COMMAS_NOT_ALLOWED));
                let actual = DefaultCliArgsValidator::is_valid_template_name(value);

                assert_eq!(actual, expected);
            }

            #[test]
            fn it_returns_error_for_values_with_commas_and_whitespaces() {
                let value = "r, ust";

                let expected: Result<String, String> =
                    Err(String::from(constant::error_messages::COMMAS_NOT_ALLOWED));
                let actual = DefaultCliArgsValidator::is_valid_template_name(value);

                assert_eq!(actual, expected);
            }
        }
    }

    mod is_starting_with_slash {
        use super::*;

        mod success {
            use super::*;

            #[test]
            fn it_returns_ok_for_value_starting_with_slash() {
                let value = "/valid/uri";

                let expected: Result<String, String> = Ok(String::from(value));
                let actual = DefaultCliArgsValidator::is_starting_with_slash(value);

                assert_eq!(actual, expected);
            }
        }

        mod failure {
            use super::*;

            #[test]
            fn it_returns_error_for_valid_not_starting_with_slash() {
                let expected: Result<String, String> = Err(String::from(
                    constant::error_messages::URI_WITHOUT_STARTING_SLASH,
                ));
                let actual = DefaultCliArgsValidator::is_starting_with_slash("invalid/uri");

                assert_eq!(actual, expected);
            }
        }
    }

    mod is_valid_url {
        use super::*;

        mod success {
            use super::*;

            #[test]
            fn it_returns_ok_for_valid_url() {
                let value = "https://www.hello.com";

                let expected: Result<String, String> = Ok(String::from(value));
                let actual = DefaultCliArgsValidator::is_valid_url(value);

                assert_eq!(actual, expected);
            }
        }

        mod failure {
            use super::*;

            #[rstest]
            #[case("hello", constant::error_messages::INVALID_URL)]
            #[case("https//foo.com", constant::error_messages::INVALID_URL)]
            #[case("htps:/myapis.foobar.com", constant::error_messages::INVALID_SCHEME)]
            fn it_returns_error_for_invalid_url(
                #[case] input_url: &str,
                #[case] expected_err_msg: &str,
            ) {
                let expected: Result<String, String> = Err(expected_err_msg.to_string());
                let actual = DefaultCliArgsValidator::is_valid_url(input_url);

                assert_eq!(actual, expected);
            }
        }
    }
}
