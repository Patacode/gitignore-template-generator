mod api; // public api
mod impls; // private api

mod config; // cli args configs
mod http_client; // http client to make api calls
mod validator; // cli args validators

// exposed entities
pub use api::get_call_to_gitignore_template_service;
pub use config::Args;
