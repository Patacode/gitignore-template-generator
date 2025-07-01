//! Define components to help in other modules.
//!
//! Generic place to put helper code.

use crate::core::QualifiedString;

pub fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + c.as_str(),
    }
}

pub fn insert_at(l: &mut Vec<QualifiedString>, idx: usize, val: QualifiedString) {
    let mut tail = l.split_off(idx);
    l.push(val);
    l.append(&mut tail);
}

pub fn to_char(s: &str) -> char {
    let mut chars = s.chars();
    match (chars.next(), chars.next()) {
        (Some(c), None) => c,
        _ => '\u{000}',
    }
}
