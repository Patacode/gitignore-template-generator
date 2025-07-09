//! Define components to process cli args.
pub use crate::runner::impls::start;
use crate::{
    core::{
        GitignoreTemplateManager, RemoteGitignoreTemplateManager, TemplateFactory, TemplateManager,
    },
    parser::ClapArgsParser,
};

mod impls;

#[cfg(test)]
mod tests;

pub type RunnerCallback<T, P> = fn(TemplateManagerRunner<T>, P);
pub type MixedRunnerCallback = RunnerCallback<GitignoreTemplateManager, ClapArgsParser>;
pub type RemoteRunnerCallback = RunnerCallback<RemoteGitignoreTemplateManager, ClapArgsParser>;

pub type MixedRunner = TemplateManagerRunner<GitignoreTemplateManager>;
pub type RemoteRunner = TemplateManagerRunner<RemoteGitignoreTemplateManager>;

#[derive(Default)]
pub struct TemplateManagerRunner<F: TemplateFactory<dyn TemplateManager>> {
    _phantom_f: std::marker::PhantomData<F>,
}
