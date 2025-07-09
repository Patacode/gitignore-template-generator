use std::process::exit;

use gitignore_template_generator::{
    core::{ProgramExit, QualifiedString},
    printer::{Data, DataPrinter, DefaultDataPrinter},
    runner::start,
};

fn main() {
    start(|runner, parser| match runner.exec(&parser) {
        Ok(output) => handle_success(&output),
        Err(error) => handle_failure(&error),
    });
}

fn handle_success(output: &QualifiedString) {
    DefaultDataPrinter::pp(&Data::QualifiedString(output))
}

fn handle_failure(error: &ProgramExit) {
    DefaultDataPrinter::pp(&Data::ProgramExit(error));
    exit(error.exit_status);
}
