use crate::core::{ProgramExit, QualifiedString};

pub enum Data<'a> {
    QualifiedString(&'a QualifiedString),
    ProgramExit(&'a ProgramExit),
}

pub fn pp(data: Data) {
    match data {
        Data::QualifiedString(qs) => println!("{}", qs.value),
        Data::ProgramExit(pe) => eprintln!("{}", pe.message),
    }
}
