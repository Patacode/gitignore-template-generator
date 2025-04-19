use std::process::exit;

use gitignore_template_generator::{
    GitignoreTemplateGenerator, TemplateGenerator,
    config::{ArgsParser, DefaultArgsParser},
    http_client::UreqClient,
};

fn main() {
    let cli_args = DefaultArgsParser::parse();
    let server_url = cli_args.server_url;
    let template_names = cli_args.template_names.join(",");

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
