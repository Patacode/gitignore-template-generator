use std::{collections::HashSet, io::ErrorKind};

use super::{
    ExitKind, ProgramExit, QualifiedString, StringKind, TemplateGenerator,
    TemplateLister,
};
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
pub struct GitignoreTemplateManager<'a> {
    template_managers: &'a [&'a dyn TemplateGenerator],
}

/// Manager of gitignore templates using local filesystem.
///
/// It uses a directory on local fs as *search database*.
///
/// Whatever the action performed through it, it will always first try to
/// locate template dir using `GITIGNORE_TEMPLATE_GENERATOR_HOME` env var, and
/// if not set, will then use provided `default_template_dir`.
pub struct LocalGitignoreTemplateManager<'a> {
    /// The fallback directory in which templates
    /// are stored. Will be used in case `GITIGNORE_TEMPLATE_GENERATOR_HOME`
    /// env var is not set.
    default_template_dir: &'a str,
}

/// Manager of gitignore templates using remote API.
///
/// The templates are managed via HTTP calls using the given `http_client`.
pub struct RemoteGitignoreTemplateManager<'a> {
    /// The http client to be used to make the API call.
    http_client: &'a dyn HttpClient,

    /// The endpoint URI to generate templates
    /// (defaults to [`crate::constant::template_manager::GENERATOR_URI`]
    /// if None).
    generator_endpoint_uri: Option<&'a str>,

    /// The endpoint URI to list templates (defaults to
    /// [`crate::constant::template_manager::LISTER_URI`] if None).
    lister_endpoint_uri: Option<&'a str>,
}

impl<'a> GitignoreTemplateManager<'a> {
    pub fn new(template_managers: &'a [&'a dyn TemplateGenerator]) -> Self {
        Self { template_managers }
    }

    fn explode_and_merge_template_results(
        template_results: &[Result<QualifiedString, ProgramExit>],
    ) -> Result<QualifiedString, Vec<ProgramExit>> {
        let mut result: String = String::new();
        let mut errors: Vec<ProgramExit> = Vec::new();

        for template_result in template_results.iter() {
            match template_result {
                Ok(template) => {
                    if !errors.is_empty() {
                        continue;
                    }

                    if template.value.is_empty() {
                        continue;
                    }

                    if let StringKind::Local = template.kind {
                        result.push_str("## LOCAL\n\n");
                    } else {
                        result.push_str("## REMOTE\n\n");
                    }

                    result.push_str(&template.value);
                    result.push_str("\n\n");
                }
                Err(error) => errors.push(error.clone()),
            }
        }

        if errors.is_empty() {
            Ok(QualifiedString {
                value: result.trim_end().to_string(),
                kind: StringKind::Mixed,
            })
        } else {
            Err(errors)
        }
    }

    fn map_lines_to_qstrs(
        value: &str,
        kind: StringKind,
    ) -> Vec<QualifiedString> {
        value
            .lines()
            .map(|tt| QualifiedString {
                value: tt.to_string(),
                kind,
            })
            .collect()
    }

    fn smap_lines_to_qstrs(
        value: &str,
        kind: StringKind,
        skip: usize,
    ) -> Vec<QualifiedString> {
        value
            .lines()
            .skip(skip)
            .map(|tt| QualifiedString {
                value: tt.to_string(),
                kind,
            })
            .collect()
    }

    fn explode_and_merge_list_results(
        list_results: &[Result<QualifiedString, ProgramExit>],
    ) -> Result<Vec<QualifiedString>, Vec<ProgramExit>> {
        let mut lines_res: Vec<QualifiedString> = Vec::new();
        let mut errors: Vec<ProgramExit> = Vec::new();

        let flist_res = list_results.first().unwrap();
        match flist_res {
            Ok(list) if !list.value.is_empty() => lines_res
                .append(&mut Self::map_lines_to_qstrs(&list.value, list.kind)),
            Err(error) => errors.push(error.clone()),
            _ => {}
        }

        for list_res in list_results.iter().skip(1) {
            match list_res {
                Ok(list) if errors.is_empty() && !list.value.is_empty() => {
                    // got list lines but nothing in lines res yet
                    if lines_res.is_empty() {
                        lines_res.append(&mut Self::map_lines_to_qstrs(
                            &list.value,
                            list.kind,
                        ));
                        continue;
                    }

                    // got first line lex higher than last lines res
                    let flist_line = list.value.lines().next().unwrap();
                    if flist_line >= lines_res.last().unwrap().value.as_str() {
                        lines_res.append(&mut Self::map_lines_to_qstrs(
                            &list.value,
                            list.kind,
                        ));
                        continue;
                    }

                    // inserting list lines
                    for (i, list_line) in list.value.lines().enumerate() {
                        let lex_high_idx = lines_res
                            .iter()
                            .position(|line| list_line <= line.value.as_str());

                        if let Some(idx) = lex_high_idx {
                            helper::insert_at(
                                &mut lines_res,
                                idx,
                                QualifiedString {
                                    value: list_line.to_string(),
                                    kind: list.kind,
                                },
                            );
                        } else {
                            lines_res.append(&mut Self::smap_lines_to_qstrs(
                                &list.value,
                                list.kind,
                                i,
                            ));
                            break;
                        }
                    }
                }
                Err(error) => errors.push(error.clone()),
                _ => {}
            }
        }

        if errors.is_empty() {
            Ok(lines_res)
        } else {
            Err(errors)
        }
    }

    fn build_error(errors: &Vec<ProgramExit>) -> ProgramExit {
        let mut final_exit_status = 0;
        let mut final_message = String::new();
        let mut final_styled_message = String::new();

        for error in errors {
            final_exit_status += error.exit_status;
            final_message.push_str(&error.message);
            final_message.push('\n');
            if let Some(styled_message) = &error.styled_message {
                final_styled_message.push_str(styled_message);
                final_styled_message.push('\n');
            }
        }

        let final_styled_message = if final_styled_message.is_empty() {
            None
        } else {
            Some(final_styled_message.trim_end().to_string())
        };

        ProgramExit {
            message: final_message.trim_end().to_string(),
            exit_status: final_exit_status,
            styled_message: final_styled_message,
            kind: ExitKind::Error,
        }
    }

    fn postprocess_template_list_result(template_list_result: &str) -> String {
        template_list_result
            .lines()
            .map(|line| {
                if let Some(stripped) = line.strip_prefix("*") {
                    stripped
                } else {
                    line
                }
            })
            .map(|line| format!("{line}\n"))
            .collect::<String>()
            .trim_end()
            .to_string()
    }
}

impl<'a> LocalGitignoreTemplateManager<'a> {
    pub fn new(default_template_dir: Option<&'a str>) -> Self {
        Self {
            default_template_dir: default_template_dir.unwrap_or(""),
        }
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
                    let error_message = match error.kind() {
                        ErrorKind::NotFound => {
                            constant::error_messages::UNSUPPORTED_TEMPLATE
                                .into()
                        }
                        _ => error.to_string(),
                    };

                    return Err(ProgramExit {
                        message: format!(
                            "{}: {}",
                            constant::error_messages::LOCAL_GENERATION,
                            error_message
                        ),
                        exit_status: constant::exit_status::GENERIC,
                        styled_message: None,
                        kind: ExitKind::Error,
                    });
                }
            };
        }

        Ok(templates)
    }
}

impl<'a> RemoteGitignoreTemplateManager<'a> {
    pub fn new(
        http_client: &'a dyn HttpClient,
        generator_endpoint_uri: Option<&'a str>,
        lister_endpoint_uri: Option<&'a str>,
    ) -> Self {
        Self {
            http_client,
            generator_endpoint_uri,
            lister_endpoint_uri,
        }
    }

    fn parse_template_list_from_api(template_list: String) -> String {
        template_list.replace(',', "\n")
    }
}

impl TemplateLister for GitignoreTemplateManager<'_> {
    fn list(&self) -> Result<QualifiedString, ProgramExit> {
        let template_list_results: Vec<Result<QualifiedString, ProgramExit>> =
            self.template_managers
                .iter()
                .map(|template_manager| template_manager.list())
                .collect();

        if template_list_results.is_empty() {
            return Ok(QualifiedString {
                value: String::new(),
                kind: StringKind::Mixed,
            });
        }

        match Self::explode_and_merge_list_results(&template_list_results) {
            Ok(result) => Ok(QualifiedString {
                value: result
                    .iter()
                    .map(|qstr| {
                        if qstr.kind == StringKind::Local {
                            format!("*{}\n", qstr.value)
                        } else {
                            qstr.value.clone() + "\n"
                        }
                    })
                    .collect::<String>()
                    .trim_end()
                    .to_string(),
                kind: StringKind::Mixed,
            }),
            Err(errors) => Err(Self::build_error(&errors)),
        }
    }
}

impl TemplateGenerator for GitignoreTemplateManager<'_> {
    fn generate(
        &self,
        template_names: &[String],
    ) -> Result<QualifiedString, ProgramExit> {
        let mut processed_templates: HashSet<String> = HashSet::new();
        let template_results: Vec<Result<QualifiedString, ProgramExit>> = self
            .template_managers
            .iter()
            .map(|template_manager| match template_manager.list() {
                Ok(supported_templates) => {
                    let templates_to_process: Vec<String> = supported_templates
                        .value
                        .lines()
                        .filter(|line| {
                            template_names.contains(&line.to_string())
                        })
                        .map(|line| line.to_string())
                        .collect();

                    let result =
                        template_manager.generate(&templates_to_process);
                    if result.is_ok() {
                        templates_to_process.iter().for_each(|template_name| {
                            processed_templates
                                .insert(template_name.to_string());
                        });
                    }
                    result
                }
                Err(error) => Err(error),
            })
            .collect();

        if template_results.is_empty() {
            return Ok(QualifiedString {
                value: String::new(),
                kind: StringKind::Mixed,
            });
        }

        if template_results.iter().all(|result| result.clone().is_ok())
            && !HashSet::from_iter(
                template_names.iter().map(|tt| tt.to_string()),
            )
            .difference(&processed_templates)
            .collect::<HashSet<&String>>()
            .is_empty()
        {
            return Err(ProgramExit {
                message: constant::error_messages::UNSUPPORTED_TEMPLATE
                    .to_string(),
                exit_status: constant::exit_status::GENERIC,
                styled_message: None,
                kind: ExitKind::Error,
            });
        }

        match Self::explode_and_merge_template_results(&template_results) {
            Ok(result) => Ok(result),
            Err(errors) => Err(Self::build_error(&errors)),
        }
    }

    fn generate_with_template_check(
        &self,
        template_names: &[String],
    ) -> Result<QualifiedString, ProgramExit> {
        let available_templates = self.list();
        available_templates.clone()?;

        let invalid_template_names = find_invalid_templates(
            &Self::postprocess_template_list_result(
                &available_templates.unwrap().value,
            ),
            template_names,
        );

        if invalid_template_names.is_empty() {
            let template_results: Vec<Result<QualifiedString, ProgramExit>> =
                self.template_managers
                    .iter()
                    .map(|tpl_mgr| match tpl_mgr.list() {
                        Ok(list) => {
                            let templates_to_process: Vec<String> = list
                                .value
                                .lines()
                                .filter(|line| {
                                    template_names.contains(&line.to_string())
                                })
                                .map(|line| line.to_string())
                                .collect();

                            tpl_mgr.generate(&templates_to_process)
                        }
                        Err(error) => Err(error),
                    })
                    .collect();

            if template_results.is_empty() {
                return Ok(QualifiedString {
                    value: String::new(),
                    kind: StringKind::Mixed,
                });
            }

            match Self::explode_and_merge_template_results(&template_results) {
                Ok(result) => Ok(result),
                Err(errors) => Err(Self::build_error(&errors)),
            }
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

impl TemplateLister for LocalGitignoreTemplateManager<'_> {
    fn list(&self) -> Result<QualifiedString, ProgramExit> {
        let template_dir = match std::env::var(HOME_ENV_VAR) {
            Ok(directory_path) => directory_path,
            Err(_) => self.default_template_dir.into(),
        };

        let directory_handler = DirectoryHandler::new(&template_dir);
        match directory_handler.list_files() {
            Ok(mut template_names) => {
                template_names.sort();
                Ok(QualifiedString {
                    value: template_names.join("\n"),
                    kind: StringKind::Local,
                })
            }
            Err(error) => match error.kind() {
                ErrorKind::NotFound => Ok(QualifiedString {
                    value: String::new(),
                    kind: StringKind::Local,
                }),
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
}

impl TemplateGenerator for LocalGitignoreTemplateManager<'_> {
    fn generate(
        &self,
        template_names: &[String],
    ) -> Result<QualifiedString, ProgramExit> {
        let template_dir = match std::env::var(HOME_ENV_VAR) {
            Ok(directory_path) => directory_path,
            Err(_) => self.default_template_dir.into(),
        };

        let templates = Self::map_template_names_to_their_content(
            &template_dir,
            template_names,
        )?;

        Ok(QualifiedString {
            value: templates.join("\n\n"),
            kind: StringKind::Local,
        })
    }

    fn generate_with_template_check(
        &self,
        template_names: &[String],
    ) -> Result<QualifiedString, ProgramExit> {
        let available_templates = self.list();
        available_templates.clone()?;

        let invalid_template_names = find_invalid_templates(
            &available_templates.unwrap().value,
            template_names,
        );

        if invalid_template_names.is_empty() {
            self.generate(template_names)
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

impl TemplateLister for RemoteGitignoreTemplateManager<'_> {
    fn list(&self) -> Result<QualifiedString, ProgramExit> {
        let endpoint_uri = self.lister_endpoint_uri.unwrap_or(LISTER_URI);
        match self.http_client.get(endpoint_uri) {
            Ok(result) => Ok(QualifiedString {
                value: Self::parse_template_list_from_api(result),
                kind: StringKind::Remote,
            }),
            Err(error) => Err(error),
        }
    }
}

impl TemplateGenerator for RemoteGitignoreTemplateManager<'_> {
    fn generate(
        &self,
        template_names: &[String],
    ) -> Result<QualifiedString, ProgramExit> {
        if template_names.is_empty() {
            return Ok(QualifiedString {
                value: String::new(),
                kind: StringKind::Remote,
            });
        }

        let path_param = template_names.join(",");
        let endpoint_uri = self.generator_endpoint_uri.unwrap_or(GENERATOR_URI);
        let full_uri = format!("{endpoint_uri}/{path_param}");

        match self.http_client.get(&full_uri) {
            Ok(result) => Ok(QualifiedString {
                value: result,
                kind: StringKind::Remote,
            }),
            Err(error) => Err(error),
        }
    }

    fn generate_with_template_check(
        &self,
        template_names: &[String],
    ) -> Result<QualifiedString, ProgramExit> {
        let available_templates = self.list();
        available_templates.clone()?;

        let invalid_template_names = find_invalid_templates(
            &available_templates.unwrap().value,
            template_names,
        );

        if invalid_template_names.is_empty() {
            self.generate(template_names)
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

fn find_invalid_templates(available: &str, provided: &[String]) -> Vec<String> {
    let available_set: HashSet<&str> = available.lines().collect();

    provided
        .iter()
        .filter(|name| !available_set.contains(name.as_str()))
        .map(|name| name.into())
        .collect()
}
