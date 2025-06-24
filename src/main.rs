use std::process::exit;

use gitignore_template_generator::{
    parser::{ArgsParser, ClapArgsParser},
    runner::get_runner,
};

// workflow:
// 1. create a runner instance using get_runner()
// 2. parse cli args using an instance of ClapArgsParser
// 3. call run method of runner with parsed cli args
// 4. create template manager through runner
// 5. process parsed cli args with template manager
// 6. parse template manager result
// 7. return the parsed result

fn main() {
    match get_runner().run(&ClapArgsParser::new().parse(std::env::args_os())) {
        Ok(output) => println!("{}", output.value),
        Err(error) => {
            eprintln!("{}", error.message);
            exit(error.exit_status);
        }
    };
}
