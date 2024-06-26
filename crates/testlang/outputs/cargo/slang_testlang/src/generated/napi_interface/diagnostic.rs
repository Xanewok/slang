// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use napi_derive::napi;

use crate::napi_interface::text_index::TextRange;

/// Severity of the compiler diagnostic.
///
/// Explicitly compatible with the LSP protocol.
#[napi(namespace = "diagnostic")]
pub enum Severity {
    Error = 1,
    Warning = 2,
    Information = 3,
    Hint = 4,
}

impl From<crate::diagnostic::Severity> for Severity {
    fn from(value: crate::diagnostic::Severity) -> Severity {
        match value {
            crate::diagnostic::Severity::Error => Self::Error,
            crate::diagnostic::Severity::Warning => Self::Warning,
            crate::diagnostic::Severity::Information => Self::Information,
            crate::diagnostic::Severity::Hint => Self::Hint,
        }
    }
}

#[napi(namespace = "diagnostic")]
pub struct Diagnostic(pub(crate) Box<dyn crate::diagnostic::Diagnostic>);

#[napi(namespace = "diagnostic")]
impl Diagnostic {
    #[napi]
    pub fn severity(&self) -> Severity {
        self.0.severity().into()
    }

    #[napi(ts_return_type = "text_index.TextRange")]
    pub fn text_range(&self) -> TextRange {
        self.0.range().into()
    }

    #[napi]
    pub fn message(&self) -> String {
        self.0.message()
    }

    #[napi]
    pub fn code(&self) -> String {
        self.0.code().to_string()
    }
}
