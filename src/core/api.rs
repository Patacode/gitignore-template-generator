pub use crate::core::impls::GitignoreTemplateGenerator;

pub trait TemplateGenerator {
    fn generate(values: &String) -> String;
}
