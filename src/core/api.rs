pub use crate::core::impls::GitignoreTemplateGenerator;
use crate::http_client::ProgramError;

pub trait TemplateGenerator {
    fn generate(values: &String) -> Result<String, ProgramError>;
}
