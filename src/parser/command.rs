//! Define components to build cli args

use clap::Arg;

mod author;
mod endpoint_uri;
mod help;
mod list;
mod server_url;
mod template_names;
mod version;

use author::AuthorClapArg;
use endpoint_uri::EndpointUriClapArg;
use help::HelpClapArg;
use list::ListClapArg;
use server_url::ServerUrlClapArg;
use template_names::TemplateNamesClapArg;
use version::VersionClapArg;

pub trait ClapArg {
    fn build() -> Arg;
}

pub fn build_clap_args() -> Vec<Arg> {
    vec![
        TemplateNamesClapArg::build(),
        ServerUrlClapArg::build(),
        EndpointUriClapArg::build(),
        ListClapArg::build(),
        HelpClapArg::build(),
        VersionClapArg::build(),
        AuthorClapArg::build(),
    ]
}
