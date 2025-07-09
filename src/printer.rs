use clap::Error;

use crate::core::{ProgramExit, QualifiedString};

mod impls;

#[cfg(test)]
mod tests;

pub enum Data<'a> {
    QualifiedString(&'a QualifiedString),
    ProgramExit(&'a ProgramExit),
    EnvVarReset(&'a str),
    EnvVarRemovalBefore(),
    EnvVarRemovalAfter(),
    DefaultTimeout(),
    ClapError(&'a Error),
    StyledClapError(&'a Error),
    Any(&'a str),
}

pub trait DataPrinter {
    fn pp(data: &Data);
    fn ppg(data: &Data) -> String;
}

pub struct DefaultDataPrinter;
