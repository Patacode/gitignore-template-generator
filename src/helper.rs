//! Define components to help in other modules.
//!
//! Generic place to put helper code.
use clap::ValueEnum;

use crate::core::QualifiedString;

mod impls;

#[cfg(test)]
mod tests;

#[derive(Clone, Copy, Debug, ValueEnum, PartialEq)]
pub enum TimeoutUnit {
    MILLISECOND,
    SECOND,
}

pub struct CliOptionName {
    pub short: &'static str,
    pub long: &'static str,
}

pub trait Utils {
    fn capitalize(s: &str) -> String;
    fn insert_at(l: &mut Vec<QualifiedString>, idx: usize, val: QualifiedString);
    fn to_char(s: &str) -> char;
}

pub struct DefaultUtils;
