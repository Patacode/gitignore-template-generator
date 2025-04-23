use std::process::exit;

use gitignore_template_generator::{
    GitignoreTemplateGenerator, TemplateGenerator,
    config::{ArgsParser, DefaultArgsParser},
    http_client::{ErrorKind, ProgramError, UreqClient},
};

fn main() {
    let cli_args = std::env::args_os();
    match DefaultArgsParser::try_parse(cli_args) {
        Ok(parsed_cli_args) => {
            let server_url = parsed_cli_args.server_url;
            let template_names = parsed_cli_args.template_names.join(",");

            let http_client = UreqClient { server_url };
            let generated_template =
                GitignoreTemplateGenerator::generate_from_api(
                    &http_client,
                    &template_names,
                );

            match generated_template {
                Ok(template) => print!("{template}"),
                Err(error) => {
                    print_error_message(&error, &error.message);
                    exit(error.exit_status);
                }
            }
        }
        Err(error) => {
            if let Some(value) = &error.styled_message {
                print_error_message(&error, value);
            } else {
                print_error_message(&error, &error.message);
            }

            exit(error.exit_status);
        }
    }
}

fn print_error_message(error: &ProgramError, message: &str) {
    if let Some(kind) = &error.error_kind {
        match kind {
            ErrorKind::VersionInfos
            | ErrorKind::HelpInfos
            | ErrorKind::AuthorInfos => println!("{message}"),
            ErrorKind::Other => eprintln!("{message}"),
        }
    } else {
        eprintln!("{message}");
    }
}
