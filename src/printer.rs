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
}

pub fn pp(data: Data) {
    match data {
        Data::QualifiedString(qs) => println!("{}", qs.value),
        Data::ProgramExit(pe) => eprintln!("{}", pe.message),
        Data::EnvVarReset(ev) => println!(
            "{}",
            help_texts::ENV_VAR_RESET
                .replace("{name}", template_manager::HOME_ENV_VAR)
                .replace("{value}", ev)
        ),
        Data::EnvVarRemovalBefore() => println!(
            "{}",
            help_texts::ENV_VAR_REMOVAL_BEFORE.replace("{name}", template_manager::HOME_ENV_VAR)
        ),
        Data::EnvVarRemovalAfter() => println!(
            "{}",
            help_texts::ENV_VAR_REMOVAL_AFTER.replace("{name}", template_manager::HOME_ENV_VAR)
        ),
        Data::Reset() => println!("{}", help_texts::RESET),
        Data::Removed() => println!("{}", help_texts::REMOVED),
        Data::TestContextDropped() => println!("{}", help_texts::TEST_CXT_DROPPED),
        Data::TestContextCreated() => println!("{}", help_texts::TEST_CTX_CREATED),
    }
}
