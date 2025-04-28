use clap::Arg;

mod author;
mod endpoint_uri;
mod help;
mod server_url;
mod template_names;
mod version;

pub use author::AuthorClapArg;
pub use endpoint_uri::EndpointUriClapArg;
pub use help::HelpClapArg;
pub use server_url::ServerUrlClapArg;
pub use template_names::TemplateNamesClapArg;
pub use version::VersionClapArg;

pub trait ClapArg {
    fn build() -> Arg;
}
