use std::collections::HashSet;

use super::{ExitKind, ProgramExit, TemplateGenerator, TemplateLister};
use crate::{
    constant::{
        self,
        template_manager::{GENERATOR_URI, LISTER_URI},
    },
    http_client::HttpClient,
};

/// Manager of gitignore templates.
///
/// It can generate and list gitignore templates.
pub struct GitignoreTemplateManager;

impl GitignoreTemplateManager {
    fn parse_template_list_from_api(template_list: String) -> String {
        template_list.replace(',', "\n")
    }

    fn find_invalid_templates<'a>(
        available: &str,
        provided: &'a [String],
    ) -> Vec<&'a str> {
        let available_set: HashSet<&str> = available.lines().collect();
        provided
            .iter()
            .filter(|name| !available_set.contains(name.as_str()))
            .map(|name| name.as_str())
            .collect()
    }
}

impl TemplateGenerator for GitignoreTemplateManager {
    /// Generates a gitignore template matching given template names.
    ///
    /// * Each associated gitignore template will be merged into one big string
    /// * A 404 error will be returned if any of the given template names isn't
    ///     supported by the underlying template generator API.
    ///
    /// See [`TemplateGenerator::generate_from_api`] for more infos.
    fn generate_from_api(
        http_client: &impl HttpClient,
        endpoint_uri: Option<&str>,
        template_names: &[String],
    ) -> Result<String, ProgramExit> {
        let path_param = template_names.join(",");
        let endpoint_uri = endpoint_uri.unwrap_or(GENERATOR_URI);
        let full_uri = format!("{endpoint_uri}/{path_param}");

        http_client.get(&full_uri)
    }

    fn generate_from_api_with_template_check(
        http_client: &impl HttpClient,
        generator_endpoint_uri: Option<&str>,
        lister_endpoint_uri: Option<&str>,
        template_names: &[String],
    ) -> Result<String, ProgramExit> {
        let available_templates =
            Self::list_from_api(http_client, lister_endpoint_uri);

        if available_templates.is_err() {
            return Err(available_templates.unwrap_err());
        }

        let invalid_template_names = Self::find_invalid_templates(
            &available_templates.unwrap(),
            template_names,
        );

        if invalid_template_names.is_empty() {
            Self::generate_from_api(
                http_client,
                generator_endpoint_uri,
                template_names,
            )
        } else {
            Err(ProgramExit {
                message: constant::error_messages::INEXISTENT_TEMPLATE_NAMES
                    .replace(
                        "{templates}",
                        invalid_template_names.join(", ").as_str(),
                    ),
                exit_status: constant::exit_status::GENERIC,
                styled_message: None,
                kind: ExitKind::Error,
            })
        }
    }
}

impl TemplateLister for GitignoreTemplateManager {
    fn list_from_api(
        http_client: &impl HttpClient,
        endpoint_uri: Option<&str>,
    ) -> Result<String, ProgramExit> {
        let endpoint_uri = endpoint_uri.unwrap_or(LISTER_URI);
        match http_client.get(endpoint_uri) {
            Ok(result) => Ok(Self::parse_template_list_from_api(result)),
            Err(error) => Err(error),
        }
    }
}
