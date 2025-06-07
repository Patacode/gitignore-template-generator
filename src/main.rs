use std::{process::exit, time::Duration};

use gitignore_template_generator::{
    core::{
        ProgramExit, RemoteGitignoreTemplateManager, TemplateGenerator,
        TemplateLister,
    },
    http_client::UreqHttpClient,
    parser::{Args, ArgsParser, ClapArgsParser, TimeoutUnit},
};

cfg_if::cfg_if! {
    if #[cfg(feature = "local_templating")] {
        use gitignore_template_generator::{
            constant,
            core::{
                ExitKind,
                GitignoreTemplateManager,
                LocalGitignoreTemplateManager,
                QualifiedString,
            }
        };

        pub fn get_result(
            parsed_cli_args: &Args
        ) -> Result<QualifiedString, ProgramExit> {
            match std::env::var("HOME") {
                Ok(home_path) => {
                    let server_url = &parsed_cli_args.server_url;
                    let generator_uri = &parsed_cli_args.generator_uri;
                    let lister_uri = &parsed_cli_args.lister_uri;
                    let template_names = &parsed_cli_args.template_names;
                    let timeout_unit = parsed_cli_args.timeout_unit;
                    let global_timeout = if timeout_unit == TimeoutUnit::SECOND {
                        Some(Duration::from_secs(parsed_cli_args.timeout))
                    } else {
                        Some(Duration::from_millis(parsed_cli_args.timeout))
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

                    let default_template_dir = format!("{home_path}/{}/templates", constant::template_manager::DEFAULT_HOME);
                    let local_manager = LocalGitignoreTemplateManager::new(Some(default_template_dir.as_str()));
                    let managers: [&dyn TemplateGenerator; 2] = [&local_manager, &remote_manager];
                    let manager = GitignoreTemplateManager::new(&managers);

                    if parsed_cli_args.show_list {
                        manager.list()
                    } else if parsed_cli_args.check_template_names {
                        manager.generate_with_template_check(template_names)
                    } else {
                        manager.generate(template_names)
                    }
                },
                Err(error) => Err(ProgramExit {
                    message: format!("An error occured when trying to read $HOME, which is required for local generation: {}", error),
                    exit_status: constant::exit_status::GENERIC,
                    styled_message: None,
                    kind: ExitKind::Error,
                }),
            }
        }
    } else {
        use gitignore_template_generator::core::QualifiedString;

        pub fn get_result(
            parsed_cli_args: &Args
        ) -> Result<QualifiedString, ProgramExit> {
            let server_url = &parsed_cli_args.server_url;
            let generator_uri = &parsed_cli_args.generator_uri;
            let lister_uri = &parsed_cli_args.lister_uri;
            let template_names = &parsed_cli_args.template_names;
            let timeout_unit = parsed_cli_args.timeout_unit;
            let global_timeout = if timeout_unit == TimeoutUnit::SECOND {
                Some(Duration::from_secs(parsed_cli_args.timeout))
            } else {
                Some(Duration::from_millis(parsed_cli_args.timeout))
            };

            let http_client = UreqHttpClient {
                server_url: server_url.to_string(),
                global_timeout,
            };

            let remote_manager = RemoteGitignoreTemplateManager::new(
                &http_client,
                Some(&generator_uri),
                Some(&lister_uri),
            );

            if parsed_cli_args.show_list {
                remote_manager.list()
            } else if parsed_cli_args.check_template_names {
                remote_manager.generate_with_template_check(&template_names)
            } else {
                remote_manager.generate(&template_names)
            }
        }
    }
}

fn main() {
    let cli_args = std::env::args_os();
    let cli_args_parser = ClapArgsParser::new();
    let parsed_cli_args = cli_args_parser.parse(cli_args);

    let result = get_result(&parsed_cli_args);

    match result {
        Ok(output) if output.value.is_empty() => {
            println!("Nothing to be printed")
        }
        Ok(output) => println!("{}", output.value),
        Err(error) => {
            eprintln!("{}", error.message);
            exit(error.exit_status);
        }
    };
}
