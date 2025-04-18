pub use crate::validator::impls::DefaultCliArgsValidator;

pub trait CliArgsValidator {
    fn has_no_commas(value: &str) -> Result<String, String>;
}

#[cfg(test)]
mod tests {
    use super::*;

    mod has_no_commas {
        use super:: *;

        #[test]
        fn it_returns_ok_for_comma_free_values() {
            let value = "rust";
            let expected: Result<String, String> = Ok(String::from(value));
            let actual = DefaultCliArgsValidator::has_no_commas(value);

            assert_eq!(actual, expected);
        }

        #[test]
        fn it_returns_error_for_values_with_commas() {
            let value = "r,ust";
            let expected: Result<String, String> =
                Err(String::from("Commas are not allowed in template names"));
            let actual = DefaultCliArgsValidator::has_no_commas(value);

            assert_eq!(actual, expected);
        }
    }
}
