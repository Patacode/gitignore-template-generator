//! Define components to build cli args

use clap::Arg;

mod author;
mod check;
mod generator_uri;
mod help;
mod list;
mod lister_uri;
mod server_url;
mod template_names;
mod version;

use author::AuthorClapArg;
use check::CheckClapArg;
use generator_uri::GeneratorUriClapArg;
use help::HelpClapArg;
use list::ListClapArg;
use lister_uri::ListerUriClapArg;
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
        GeneratorUriClapArg::build(),
        ListerUriClapArg::build(),
        ListClapArg::build(),
        CheckClapArg::build(),
        HelpClapArg::build(),
        VersionClapArg::build(),
        AuthorClapArg::build(),
    ]
}
