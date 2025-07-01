use std::process::exit;

use gitignore_template_generator::{
    core::{ProgramExit, QualifiedString},
    printer::{Data, pp},
    runner::start,
};

fn main() {
    start(|runner, parser| match runner.exec(&parser) {
        Ok(output) => handle_success(&output),
        Err(error) => handle_failure(&error),
    });
}

fn handle_success(output: &QualifiedString) {
    pp(Data::QualifiedString(output))
}

fn handle_failure(error: &ProgramExit) {
    pp(Data::ProgramExit(error));
    exit(error.exit_status);
}
