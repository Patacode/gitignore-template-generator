mod config; // cli args configs
mod core; // core api
mod http_client; // http client to make api calls
mod validator; // cli args validators

// exposed entities
pub use config::Args;
pub use core::get_call_to_gitignore_template_service;
