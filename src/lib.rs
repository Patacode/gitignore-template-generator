mod api; // public api
mod impls; // private api

mod config; // cli args configs
mod validator; // cli args validators
mod http_client; // http client to make api calls

// exposed entities
pub use api::get_call_to_gitignore_template_service;
pub use config::Args;
