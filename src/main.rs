use std::process::exit;

use gitignore_template_generator::{
    GitignoreTemplateGenerator, TemplateGenerator,
    config::{ArgsParser, DefaultArgsParser},
    http_client::UreqClient,
};

fn main() {
    let args = DefaultArgsParser::parse();

    let client = UreqClient {
        server_url: args.server_url,
    };
    let args = args.template_names.join(",");
    let result = GitignoreTemplateGenerator::generate_from_api(&client, &args);

    match result {
        Ok(body) => print!("{body}"),
        Err(error) => {
            eprintln!("{}", error.message);
            exit(error.exit_status);
        }
    }
}
