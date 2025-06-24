use std::process::exit;

use gitignore_template_generator::{
    core::ProgramExit,
    parser::{Args, ArgsParser, ClapArgsParser},
    runner::Runner,
};

cfg_if::cfg_if! {
    if #[cfg(feature = "local_templating")] {
        use gitignore_template_generator::{
            core::QualifiedString,
            runner::LocalRemoteRunner
        };

        pub fn get_result(
            parsed_cli_args: &Args
        ) -> Result<QualifiedString, ProgramExit> {
            LocalRemoteRunner::run(parsed_cli_args)
        }
    } else {
        use gitignore_template_generator::{
            core::QualifiedString,
            runner::RemoteRunner
        };

        pub fn get_result(
            parsed_cli_args: &Args
        ) -> Result<QualifiedString, ProgramExit> {
            RemoteRunner::run(parsed_cli_args)
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
