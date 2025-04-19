pub use crate::validator::impls::DefaultCliArgsValidator;

pub trait CliArgsValidator {
    fn has_no_commas(value: &str) -> Result<String, String>;
}

#[cfg(test)]
mod tests {
    use super::*;

    mod default_cli_args_validator {
        use super::*;

        mod has_no_commas {
            use super::*;

            mod success {
                use super::*;

                #[test]
                fn it_returns_ok_for_comma_free_values() {
                    let value = "rust";

                    let expected: Result<String, String> =
                        Ok(String::from(value));
                    let actual = DefaultCliArgsValidator::has_no_commas(value);

                    assert_eq!(actual, expected);
                }
            }

            mod failure {
                use crate::constant;

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
    }
}
