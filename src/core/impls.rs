use std::{collections::HashSet, io::ErrorKind};

use super::{ExitKind, ProgramExit, TemplateGenerator, TemplateLister};
use crate::{
    constant::{
        self,
        template_manager::{GENERATOR_URI, HOME_ENV_VAR, LISTER_URI},
    },
    fs::{DirectoryHandler, FileSystemHandler},
    helper,
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

    fn map_template_names_to_their_content(
        template_dir: &str,
        template_names: &[String],
    ) -> Result<Vec<String>, ProgramExit> {
        let directory_handler = DirectoryHandler::new(template_dir);
        let mut templates = Vec::new();

        for template_name in template_names {
            let file_name = format!("{template_name}.txt");
            match directory_handler.fetch_content(&file_name) {
                Ok(template) => templates.push(format!(
                    "### *{} ###\n{}",
                    helper::capitalize(template_name),
                    template
                )),
                Err(error) => {
                    return match error.kind() {
                        ErrorKind::NotFound => Err(ProgramExit {
                            message: format!(
                                "{}: {}",
                                constant::error_messages::LOCAL_GENERATION,
                                constant::error_messages::UNSUPPORTED_TEMPLATE
                            ),
                            exit_status: constant::exit_status::GENERIC,
                            styled_message: None,
                            kind: ExitKind::Error,
                        }),
                        _ => Err(ProgramExit {
                            message: format!(
                                "{}: {}",
                                constant::error_messages::LOCAL_GENERATION,
                                error
                            ),
                            exit_status: constant::exit_status::GENERIC,
                            styled_message: None,
                            kind: ExitKind::Error,
                        }),
                    };
                }
            };
        }

        Ok(templates)
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
        available_templates.clone()?;

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

    fn generate_locally(
        default_template_dir: &str,
        template_names: &[String],
    ) -> Result<String, ProgramExit> {
        let template_dir = match std::env::var(HOME_ENV_VAR) {
            Ok(directory_path) => directory_path,
            Err(_) => default_template_dir.into(),
        };

        let templates = Self::map_template_names_to_their_content(
            &template_dir,
            template_names,
        )?;

        Ok(templates.join("\n\n"))
    }
}

impl TemplateLister for GitignoreTemplateManager {
    fn list_locally(default_template_dir: &str) -> Result<String, ProgramExit> {
        let template_dir = match std::env::var(HOME_ENV_VAR) {
            Ok(directory_path) => directory_path,
            Err(_) => default_template_dir.into(),
        };
        let directory_handler = DirectoryHandler::new(&template_dir);
        match directory_handler.list_files() {
            Ok(template_names) => Ok(template_names
                .iter()
                .map(|template_name| format!("*{template_name}"))
                .collect::<Vec<String>>()
                .join("\n")),
            Err(error) => match error.kind() {
                ErrorKind::NotFound => Ok(String::from("")),
                _ => Err(ProgramExit {
                    message: format!(
                        "{}: {}",
                        constant::error_messages::LOCAL_LISTING,
                        error
                    ),
                    exit_status: constant::exit_status::GENERIC,
                    styled_message: None,
                    kind: ExitKind::Error,
                }),
            },
        }
    }

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
