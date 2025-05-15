use std::{process::exit, time::Duration};

use gitignore_template_generator::{
    GitignoreTemplateManager, TemplateGenerator, TemplateLister,
    http_client::UreqHttpClient,
    parser::{ArgsParser, ClapArgsParser, TimeoutUnit},
};

fn main() {
    let cli_args = std::env::args_os();
    let cli_args_parser = ClapArgsParser::new();
    let parsed_cli_args = cli_args_parser.parse(cli_args);

    let server_url = parsed_cli_args.server_url;
    let generator_uri = parsed_cli_args.generator_uri;
    let lister_uri = parsed_cli_args.lister_uri;
    let template_names = parsed_cli_args.template_names;
    let timeout_unit = parsed_cli_args.timeout_unit;
    let global_timeout = if timeout_unit == TimeoutUnit::SECOND {
        Some(Duration::from_secs(parsed_cli_args.timeout))
    } else {
        Some(Duration::from_millis(parsed_cli_args.timeout))
    };

    let http_client = UreqHttpClient {
        server_url,
        global_timeout,
    };
    let result = if parsed_cli_args.show_list {
        GitignoreTemplateManager::list_from_api(&http_client, Some(&lister_uri))
    } else if parsed_cli_args.check_template_names {
        GitignoreTemplateManager::generate_from_api_with_template_check(
            &http_client,
            Some(&generator_uri),
            Some(&lister_uri),
            &template_names,
        )
    } else {
        GitignoreTemplateManager::generate_from_api(
            &http_client,
            Some(&generator_uri),
            &template_names,
        )
    };

    match result {
        Ok(output) => println!("{output}"),
        Err(error) => {
            eprintln!("{}", error.message);
            exit(error.exit_status);
        }
    };
}
