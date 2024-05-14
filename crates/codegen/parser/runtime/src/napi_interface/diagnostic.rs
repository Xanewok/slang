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

#[napi(object, namespace = "diagnostic")]
pub struct Diagnostic {
    pub severity: Severity,
    #[napi(ts_type = "text_index.TextRange")]
    pub range: TextRange,
    pub message: String,
    pub code: String,
}
