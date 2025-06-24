pub use crate::runner::impls::{LocalRemoteRunner, RemoteRunner};
use crate::{
    core::{ProgramExit, QualifiedString},
    parser::Args,
};

pub trait Runner {
    fn run(args: &Args) -> Result<QualifiedString, ProgramExit>;
}
