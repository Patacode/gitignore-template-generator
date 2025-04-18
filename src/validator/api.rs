pub use crate::validator::impls::DefaultCliArgsValidator;

pub trait CliArgsValidator {
    fn has_no_commas(value: &str) -> Result<String, String>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_checks_that_value_without_commas_has_no_commas() {
        let value = "rust";
        let expected: Result<String, String> = Ok(String::from(value));
        let actual = DefaultCliArgsValidator::has_no_commas(value);

        assert_eq!(actual, expected);
    }

    #[test]
    fn it_checks_that_value_with_commas_has_no_commas() {
        let value = "r,ust";
        let expected: Result<String, String> =
            Err(String::from("Commas are not allowed in template names"));
        let actual = DefaultCliArgsValidator::has_no_commas(value);

        assert_eq!(actual, expected);
    }
}
