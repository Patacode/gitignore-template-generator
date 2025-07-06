use clap::Error;

use crate::{
    constant::{help_texts, template_manager},
    core::{ProgramExit, QualifiedString},
};

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

pub fn pp(data: &Data) {
    let value = ppg(data);
    match data {
        Data::ProgramExit(_) => eprintln!("{value}"),
        _ => println!("{value}"),
    }
}

pub fn ppg(data: &Data) -> String {
    match data {
        Data::QualifiedString(qs) => qs.value.clone(),
        Data::ProgramExit(pe) => pe.message.clone(),
        Data::EnvVarReset(ev) => help_texts::ENV_VAR_RESET
            .replace("{name}", template_manager::HOME_ENV_VAR)
            .replace("{value}", ev),
        Data::EnvVarRemovalBefore() => {
            help_texts::ENV_VAR_REMOVAL_BEFORE.replace("{name}", template_manager::HOME_ENV_VAR)
        }
        Data::EnvVarRemovalAfter() => {
            help_texts::ENV_VAR_REMOVAL_AFTER.replace("{name}", template_manager::HOME_ENV_VAR)
        }
        Data::DefaultTimeout() => help_texts::DEFAULT_TIMEOUT
            .replace("{second}", template_manager::TIMEOUT)
            .replace("{millis}", template_manager::TIMEOUT_MILLISECOND),
        Data::ClapError(error) => {
            help_texts::HELP_FOR_MORE_INFOS.replace("{error}", &error.render().to_string())
        }
        Data::StyledClapError(error) => help_texts::STYLED_HELP_FOR_MORE_INFOS
            .replace("{error}", &error.render().ansi().to_string()),
        Data::Any(value) => value.to_string(),
    }
}
