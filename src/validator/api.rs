pub use crate::validator::impls::DefaultCliArgsValidator;

pub trait CliArgsValidator {
    fn has_no_commas(value: &str) -> Result<String, String>;
}
