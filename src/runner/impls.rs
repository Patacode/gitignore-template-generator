use std::time::Duration;

use crate::{
    constant,
    core::{
        ExitKind, GitignoreTemplateManager, LocalGitignoreTemplateManager,
        ProgramExit, QualifiedString, RemoteGitignoreTemplateManager,
        TemplateGenerator, TemplateLister,
    },
    http_client::UreqHttpClient,
    parser::{Args, TimeoutUnit},
    runner::Runner,
};

pub struct RemoteRunner;
pub struct LocalRemoteRunner;

impl Runner for RemoteRunner {
    fn run(args: &Args) -> Result<QualifiedString, ProgramExit> {
        let server_url = &args.server_url;
        let generator_uri = &args.generator_uri;
        let lister_uri = &args.lister_uri;
        let template_names = &args.template_names;
        let timeout_unit = args.timeout_unit;
        let global_timeout = if timeout_unit == TimeoutUnit::SECOND {
            Some(Duration::from_secs(args.timeout))
        } else {
            Some(Duration::from_millis(args.timeout))
        };

        let http_client = UreqHttpClient {
            server_url: server_url.to_string(),
            global_timeout,
        };

        let remote_manager = RemoteGitignoreTemplateManager::new(
            &http_client,
            Some(generator_uri),
            Some(lister_uri),
        );

        if args.show_list {
            remote_manager.list()
        } else if args.check_template_names {
            remote_manager.generate_with_template_check(template_names)
        } else {
            remote_manager.generate(template_names)
        }
    }
}

impl Runner for LocalRemoteRunner {
    fn run(args: &Args) -> Result<QualifiedString, ProgramExit> {
        match std::env::var("HOME") {
            Ok(home_path) => {
                let server_url = &args.server_url;
                let generator_uri = &args.generator_uri;
                let lister_uri = &args.lister_uri;
                let template_names = &args.template_names;
                let timeout_unit = args.timeout_unit;
                let global_timeout = if timeout_unit == TimeoutUnit::SECOND {
                    Some(Duration::from_secs(args.timeout))
                } else {
                    Some(Duration::from_millis(args.timeout))
                };

                let http_client = UreqHttpClient {
                    server_url: server_url.to_string(),
                    global_timeout,
                };

                let remote_manager = RemoteGitignoreTemplateManager::new(
                    &http_client,
                    Some(generator_uri),
                    Some(lister_uri),
                );

                let default_template_dir = format!(
                    "{home_path}/{}/templates",
                    constant::template_manager::DEFAULT_HOME
                );
                let local_manager = LocalGitignoreTemplateManager::new(Some(
                    default_template_dir.as_str(),
                ));

                let managers: [&dyn TemplateGenerator; 2] =
                    [&local_manager, &remote_manager];
                let manager = GitignoreTemplateManager::new(&managers);

                if args.show_list {
                    manager.list()
                } else if args.check_template_names {
                    manager.generate_with_template_check(template_names)
                } else {
                    manager.generate(template_names)
                }
            }
            Err(error) => Err(ProgramExit {
                message: format!(
                    "An error occured when trying to read $HOME, which is required for local generation: {}",
                    error
                ),
                exit_status: constant::exit_status::GENERIC,
                styled_message: None,
                kind: ExitKind::Error,
            }),
        }
    }
}
