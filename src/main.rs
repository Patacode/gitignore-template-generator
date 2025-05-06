use std::process::exit;

use gitignore_template_generator::{
    GitignoreTemplateGenerator, TemplateGenerator, TemplateLister, constant,
    http_client::UreqHttpClient,
    parser::{ArgsParser, ClapArgsParser},
};

fn main() {
    let cli_args = std::env::args_os();
    let cli_args_parser = ClapArgsParser::new();
    let parsed_cli_args = cli_args_parser.parse(cli_args);

    if parsed_cli_args.show_list {
        let server_url = parsed_cli_args.server_url;
        let endpoint_uri = constant::template_lister::URI;

        let http_client = UreqHttpClient { server_url };
        let template_list = GitignoreTemplateGenerator::list_from_api(
            &http_client,
            endpoint_uri,
        );

        match template_list {
            Ok(result) => println!("{result}"),
            Err(error) => {
                eprintln!("{}", error.message);
                exit(error.exit_status);
            }
        };
    } else {
        let server_url = parsed_cli_args.server_url;
        let endpoint_uri = parsed_cli_args.endpoint_uri;
        let template_names = parsed_cli_args.template_names;

        let http_client = UreqHttpClient { server_url };
        let generated_template = GitignoreTemplateGenerator::generate_from_api(
            &http_client,
            &endpoint_uri,
            &template_names,
        );

        match generated_template {
            Ok(template) => println!("{template}"),
            Err(error) => {
                eprintln!("{}", error.message);
                exit(error.exit_status);
            }
        };
    }
}
