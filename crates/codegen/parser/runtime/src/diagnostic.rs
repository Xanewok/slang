use std::error::Error;
use std::fmt::Display;

use crate::text_index::TextRange;

#[repr(u8)]
pub enum Severity {
    Error = 1,
    Warning = 2,
    Information = 3,
    Hint = 4,
}

pub trait Diagnostic: Error {
    fn range(&self) -> TextRange;
    fn code(&self) -> impl Display;
    fn severity(&self) -> Severity;
    fn message(&self) -> String;
}
