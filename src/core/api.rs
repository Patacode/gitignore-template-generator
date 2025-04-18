pub use crate::core::impls::GitignoreTemplateGenerator;
use crate::http_client::{HttpClient, ProgramError};

pub trait TemplateGenerator {
    fn generate_from_api(
        http_client: &dyn HttpClient,
        url: &str,
        values: &String,
    ) -> Result<String, ProgramError>;
}
