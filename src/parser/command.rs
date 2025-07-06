//! Define components to build cli args

use clap::{Arg, ArgMatches};

mod author;
mod check;
mod generator_uri;
mod help;
mod list;
mod lister_uri;
mod server_url;
mod template_names;
mod timeout;
mod timeout_unit;
mod version;

pub use author::AuthorClapArg;
pub use check::CheckClapArg;
pub use generator_uri::GeneratorUriClapArg;
pub use help::HelpClapArg;
pub use list::ListClapArg;
pub use lister_uri::ListerUriClapArg;
pub use server_url::ServerUrlClapArg;
pub use template_names::TemplateNamesClapArg;
pub use timeout::TimeoutClapArg;
pub use timeout_unit::TimeoutUnitClapArg;
pub use version::VersionClapArg;

pub trait ClapArg<T> {
    fn build() -> Arg;
    fn from_arg_matches(arg_matches: &ArgMatches) -> T;
}

pub fn build_clap_args() -> [Arg; 11] {
    [
        CheckClapArg::build(),
        GeneratorUriClapArg::build(),
        ListClapArg::build(),
        ListerUriClapArg::build(),
        ServerUrlClapArg::build(),
        TemplateNamesClapArg::build(),
        TimeoutClapArg::build(),
        TimeoutUnitClapArg::build(),
        HelpClapArg::build(),
        VersionClapArg::build(),
        AuthorClapArg::build(),
    ]
}
