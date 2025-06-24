//! Define components to process cli args.

use crate::{
    constant::help_messages, core::{ProgramExit, QualifiedString, TemplateFactory, TemplateManager}, parser::Args
};

pub enum Action {
    List,
    GenerateWithTemplateCheck,
    Generate,
}

pub struct TemplateManagerRunner<
    T: TemplateManager + ?Sized,
    F: TemplateFactory<T>,
> {
    _phantom_t: std::marker::PhantomData<T>,
    _phantom_f: std::marker::PhantomData<F>,
}

impl<T: TemplateManager + ?Sized, F: TemplateFactory<T>>
    TemplateManagerRunner<T, F>
{
    pub fn new() -> Self {
        Self {
            _phantom_t: std::marker::PhantomData,
            _phantom_f: std::marker::PhantomData,
        }
    }

    pub fn run(&self, args: &Args) -> Result<QualifiedString, ProgramExit> {
        let manager = F::from_args(args)?;

        let result = match args.to_action() {
            Action::List => manager.list(),
            Action::GenerateWithTemplateCheck => {
                manager.generate_with_template_check(&args.template_names)
            }
            Action::Generate => manager.generate(&args.template_names),
        };

        self.parse_result(result)
    }

    fn parse_result(
        &self,
        result: Result<QualifiedString, ProgramExit>,
    ) -> Result<QualifiedString, ProgramExit> {
        match result {
            Ok(output) if output.value.is_empty() => Ok(QualifiedString {
                value: help_messages::NOTHING_TO_BE_PRINTED.to_string(),
                kind: output.kind,
            }),
            Ok(output) => Ok(output),
            Err(error) => Err(error),
        }
    }
}

cfg_if::cfg_if! {
    if #[cfg(feature = "local_templating")] {
        use crate::core::GitignoreTemplateManager;

        pub fn get_runner() -> TemplateManagerRunner<dyn TemplateManager, GitignoreTemplateManager> {
            TemplateManagerRunner
                ::<dyn TemplateManager, GitignoreTemplateManager>
                ::new()
        }
    } else {
        use crate::core::RemoteGitignoreTemplateManager;

        pub fn get_runner() -> TemplateManagerRunner<dyn TemplateManager, RemoteGitignoreTemplateManager> {
            TemplateManagerRunner
                ::<dyn TemplateManager, RemoteGitignoreTemplateManager>
                ::new()
        }
    }
}
