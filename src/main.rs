use std::process::exit;

use gitignore_template_generator::{
    GitignoreTemplateGenerator, TemplateGenerator,
    http_client::UreqClient,
    parser::{ArgsParser, DefaultArgsParser},
};

fn main() {
    let cli_args = std::env::args_os();
    let parsed_cli_args = DefaultArgsParser::parse(cli_args);

    let server_url = parsed_cli_args.server_url;
    let template_names = parsed_cli_args.template_names;

    let http_client = UreqClient { server_url };
    let generated_template = GitignoreTemplateGenerator::generate_from_api(
        &http_client,
        &template_names,
    );

    match generated_template {
        Ok(template) => print!("{template}"),
        Err(error) => {
            eprintln!("{}", error.message);
            exit(error.exit_status);
        }
    }
}
