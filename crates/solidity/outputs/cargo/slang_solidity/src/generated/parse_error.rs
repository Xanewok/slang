// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use crate::diagnostic::{self, Diagnostic};
use crate::kinds::TokenKind;
use crate::text_index::{TextRange, TextRangeExtensions};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ParseError {
    pub(crate) text_range: TextRange,
    pub(crate) tokens_that_would_have_allowed_more_progress: Vec<TokenKind>,
}

impl ParseError {
    pub fn text_range(&self) -> &TextRange {
        &self.text_range
    }

    pub fn to_error_report(&self, source_id: &str, source: &str, with_color: bool) -> String {
        render_error_report(self, source_id, source, with_color)
    }
}

impl ParseError {
    pub(crate) fn new(
        text_range: TextRange,
        tokens_that_would_have_allowed_more_progress: Vec<TokenKind>,
    ) -> Self {
        Self {
            text_range,
            tokens_that_would_have_allowed_more_progress,
        }
    }
}

impl Error for ParseError {}
impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.tokens_that_would_have_allowed_more_progress.is_empty() {
            write!(f, "Expected end of file.")
        } else {
            let deduped = self
                .tokens_that_would_have_allowed_more_progress
                .iter()
                .collect::<BTreeSet<_>>();

            write!(f, "Expected ")?;

            for kind in deduped.iter().take(deduped.len() - 1) {
                write!(f, "{kind} or ")?;
            }
            let last = deduped.last().expect("we just checked that it's not empty");
            write!(f, "{last}.")?;

            Ok(())
        }
    }
}

impl Diagnostic for ParseError {
    fn range(&self) -> TextRange {
        self.text_range.clone()
    }

    fn code(&self) -> Box<dyn std::fmt::Display> {
        Box::new("ParseError")
    }

    fn severity(&self) -> diagnostic::Severity {
        diagnostic::Severity::Error
    }

    fn message(&self) -> String {
        ToString::to_string(&self)
    }
}

pub(crate) fn render_error_report<D: Diagnostic>(
    error: &D,
    source_id: &str,
    source: &str,
    with_color: bool,
) -> String {
    use ariadne::{Color, Config, Label, Report, ReportKind, Source};

    let kind = match error.severity() {
        diagnostic::Severity::Error => ReportKind::Error,
        diagnostic::Severity::Warning => ReportKind::Warning,
        diagnostic::Severity::Information => ReportKind::Advice,
        diagnostic::Severity::Hint => ReportKind::Advice,
    };

    let color = if with_color { Color::Red } else { Color::Unset };

    let message = error.message();

    if source.is_empty() {
        return format!("{kind}: {message}\n   ─[{source_id}:0:0]");
    }

    let range = error.range().char();

    let mut builder = Report::build(kind, source_id, range.start)
        .with_config(Config::default().with_color(with_color))
        .with_message(message);

    builder.add_label(
        Label::new((source_id, range))
            .with_color(color)
            .with_message("Error occurred here.".to_string()),
    );

    let mut result = vec![];
    builder
        .finish()
        .write((source_id, Source::from(&source)), &mut result)
        .expect("Failed to write report");

    return String::from_utf8(result)
        .expect("Failed to convert report to utf8")
        .trim()
        .to_string();
}
