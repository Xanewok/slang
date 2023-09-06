//! A simple debugger that logs the executing parser and the current input position.
//! This is helpful in the context of backtracking, as this allows introspection
//! into how much progress a given parser made in general and whether another
//! had to be retried in the event of failure.

use std::fmt;
use std::sync::RwLock;

use once_cell::sync::Lazy;

use crate::kinds::RuleKind;
use crate::parser_support::ParserContext;

pub(crate) static DEBUGGER: Lazy<RwLock<Debugger>> =
    Lazy::new(|| RwLock::new(Debugger::with_color().ignore_trivia(true)));

const PREVIEW_LEN: usize = 15;

#[derive(Copy, Clone, PartialEq, Eq, Default, Debug)]
pub enum AnsiColored {
    Yes,
    #[default]
    No,
}

#[derive(Default)]
pub struct Debugger {
    enabled: bool,
    color: AnsiColored,
    ignore_trivia: bool,
    parsers: Vec<String>,
}

impl Debugger {
    /// Creates an instance of [`Debugger`] that uses ANSI color codes when logging.
    fn with_color() -> Self {
        Debugger {
            enabled: std::env::var("SLANG_CST_DEBUG")
                .map_or(false, |v| v != "0" && !v.eq_ignore_ascii_case("false")),
            color: AnsiColored::Yes,
            ignore_trivia: false,
            parsers: Vec::default(),
        }
    }

    /// Whether to ignore trivia when logging.
    fn ignore_trivia(mut self, ignore: bool) -> Self {
        self.ignore_trivia = ignore;
        self
    }
}

impl Debugger {
    #[allow(dead_code)]
    pub fn start_parse(kind: RuleKind, version: impl Into<String>) {
        let debugger = DEBUGGER.read().unwrap();
        if !debugger.enabled {
            return;
        }

        eprintln!(
            "{start_color}Parsing {kind} in version: {version}{end_color}",
            version = version.into(),
            start_color = (debugger.color == AnsiColored::Yes)
                .then_some("\x1b[34m")
                .unwrap_or_default(),
            end_color = (debugger.color == AnsiColored::Yes)
                .then_some("\x1b[0m")
                .unwrap_or_default(),
        );
    }

    #[allow(dead_code)]
    pub fn enter_parser<'a, 'b>(
        context: &'a mut ParserContext<'b>,
        parser: impl Into<String>,
    ) -> EnterParseGuard<'a, 'b> {
        let parser_name = parser.into();

        if let Ok(debugger) = DEBUGGER.read() {
            if !debugger.ignore_trivia || !parser_name.contains("trivia") {
                let msg = format!(
                    "{start}{parser_name}{end}",
                    start = (debugger.color == AnsiColored::Yes)
                        .then_some("\x1b[1m")
                        .unwrap_or_default(),
                    end = (debugger.color == AnsiColored::Yes)
                        .then_some("\x1b[0m")
                        .unwrap_or_default()
                );

                debugger.log_by_ref(context.preview(), msg);
            }
        }

        DEBUGGER.write().unwrap().parsers.push(parser_name);

        EnterParseGuard { context }
    }

    #[allow(dead_code)]
    pub fn log(ctx: &mut ParserContext<'_>, msg: impl fmt::Display) {
        DEBUGGER.read().unwrap().log_by_ref(ctx.preview(), msg);
    }

    fn log_by_ref(&self, preview: SourcePreview, msg: impl fmt::Display) {
        if !self.enabled {
            return;
        }

        if self.ignore_trivia && self.parsers.last().map_or(false, |p| p.contains("trivia")) {
            return;
        }

        eprintln!(
            "{prefix}{indent}{msg}",
            prefix = self.log_prefix(preview),
            indent = " ".repeat(self.depth() * 2)
        );
    }

    #[inline]
    fn depth(&self) -> usize {
        self.parsers.len()
    }

    #[inline]
    fn log_prefix(&self, preview: SourcePreview) -> impl fmt::Display {
        LogPrefix {
            preview,
            depth: self.depth(),
            with_color: self.color == AnsiColored::Yes,
        }
    }
}

struct LogPrefix {
    preview: SourcePreview,
    depth: usize,
    with_color: bool,
}

impl fmt::Display for LogPrefix {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "|")?;
        if self.with_color {
            write!(f, "\x1b[2m")?;
        }
        write!(f, "{:░<15}", self.preview)?;
        if self.with_color {
            write!(f, "\x1b[0m")?;
        }
        write!(f, "|")?;
        write!(f, "[{depth:02}]", depth = self.depth)?;

        Ok(())
    }
}

pub(crate) struct SourcePreview(String);

impl fmt::Display for SourcePreview {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

impl<'a> ParserContext<'a> {
    pub(crate) fn preview(&self) -> SourcePreview {
        let preview = self.source()[self.position().utf8..]
            .chars()
            .take(PREVIEW_LEN)
            .collect::<String>();

        SourcePreview(
            preview
                .replace('\n', "␊")
                .replace('\r', "␍")
                .replace('\t', "␉")
                .replace('\0', "␀"),
        )
    }
}

pub struct EnterParseGuard<'a, 'b> {
    context: &'a mut ParserContext<'b>,
}

impl<'a, 'b> EnterParseGuard<'a, 'b> {
    pub fn context(&mut self) -> &mut ParserContext<'b> {
        self.context
    }
}

impl<'a, 'b> Drop for EnterParseGuard<'a, 'b> {
    fn drop(&mut self) {
        let parser_name = DEBUGGER.write().unwrap().parsers.pop().unwrap();

        if let Ok(debugger) = DEBUGGER.read() {
            if !debugger.ignore_trivia || !parser_name.contains("trivia") {
                let msg = format!(
                    "{start}{parser_name} (exit){end}",
                    start = (debugger.color == AnsiColored::Yes)
                        .then_some("\x1b[1m")
                        .unwrap_or_default(),
                    end = (debugger.color == AnsiColored::Yes)
                        .then_some("\x1b[0m")
                        .unwrap_or_default()
                );

                debugger.log_by_ref(SourcePreview(String::new()), msg);
            }
        }
    }
}
