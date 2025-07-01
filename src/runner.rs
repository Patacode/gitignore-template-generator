//! Define components to process cli args.

use crate::{
    core::{
        GitignoreTemplateManager, ProgramExit, QualifiedString,
        RemoteGitignoreTemplateManager, TemplateFactory, TemplateManager,
    },
    parser::{Action, ArgsParser, ClapArgsParser},
};

pub type RunnerCallback<T, P> = fn(TemplateManagerRunner<T>, P);
pub type MixedRunnerCallback =
    RunnerCallback<GitignoreTemplateManager, ClapArgsParser>;
pub type RemoteRunnerCallback =
    RunnerCallback<RemoteGitignoreTemplateManager, ClapArgsParser>;

pub type MixedRunner = TemplateManagerRunner<GitignoreTemplateManager>;
pub type RemoteRunner = TemplateManagerRunner<RemoteGitignoreTemplateManager>;

#[derive(Default)]
pub struct TemplateManagerRunner<F: TemplateFactory<dyn TemplateManager>> {
    _phantom_f: std::marker::PhantomData<F>,
}

impl<F: TemplateFactory<dyn TemplateManager>> TemplateManagerRunner<F> {
    pub fn new() -> Self {
        Self {
            _phantom_f: std::marker::PhantomData,
        }
    }

    pub fn exec(
        &self,
        parser: &impl ArgsParser,
    ) -> Result<QualifiedString, ProgramExit> {
        let args = parser.parse(std::env::args_os());
        let manager = F::from_args(&args)?;

        let result = match args.to_action() {
            Action::List => manager.list(),
            Action::RobustGenerate => {
                manager.generate_with_template_check(&args.template_names)
            }
            Action::Generate => manager.generate(&args.template_names),
        };

        self.parse_result(&result)
    }

    fn parse_result(
        &self,
        result: &Result<QualifiedString, ProgramExit>,
    ) -> Result<QualifiedString, ProgramExit> {
        match result {
            Ok(output) if output.value.is_empty() => {
                Ok(QualifiedString::empty(output.kind))
            }
            Ok(output) => Ok(output.clone()),
            Err(error) => Err(error.clone()),
        }
    }
}

pub fn get_parser() -> ClapArgsParser {
    ClapArgsParser::new()
}

cfg_if::cfg_if! {
    if #[cfg(feature = "local_templating")] {
        pub fn start(callback: MixedRunnerCallback) {
            callback(MixedRunner::new(), get_parser());
        }
    } else {
        pub fn start(callback: RemoteRunnerCallback) {
            callback(RemoteRunner::new(), get_parser());
        }
    }
}
