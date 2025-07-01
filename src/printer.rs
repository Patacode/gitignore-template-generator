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
    Reset(),
    Removed(),
    TestContextDropped(),
    TestContextCreated(),
    DefaultTimeout(),
}

pub fn pp(data: &Data) {
    let value = ppg(&data);
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
        Data::Reset() => help_texts::RESET.to_string(),
        Data::Removed() => help_texts::REMOVED.to_string(),
        Data::TestContextDropped() => help_texts::TEST_CXT_DROPPED.to_string(),
        Data::TestContextCreated() => help_texts::TEST_CTX_CREATED.to_string(),
        Data::DefaultTimeout() => help_texts::DEFAULT_TIMEOUT
            .replace("{second}", template_manager::TIMEOUT)
            .replace("{millis}", template_manager::TIMEOUT_MILLISECOND),
    }
}
