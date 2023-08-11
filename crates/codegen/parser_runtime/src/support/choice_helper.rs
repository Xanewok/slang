use std::ops::ControlFlow;

use super::{super::text_index::TextIndex, ParserFlow, ParserResult, Stream};

// The choice is finished when we have a full match.
macro_rules! finished_state {
    () => {
        ParserResult::Match(..) | ParserResult::PrattOperatorMatch(..)
    };
}

/// Starting from a given position in the stream, this helper will try to pick (and remember) a best match. Settles on
/// a full match if possible, otherwise on the best incomplete match.
#[must_use]
pub struct ChoiceHelper {
    result: ParserResult,
    start_position: TextIndex,
}

impl ChoiceHelper {
    pub fn new(stream: &mut Stream) -> Self {
        Self {
            result: ParserResult::no_match(vec![]),
            start_position: stream.position(),
        }
    }

    /// Whether the choice has found a full match
    pub fn is_done(&self) -> bool {
        matches!(self.result, finished_state!())
    }

    /// Accumulates a new result - if it's a better match, then it's stored, otherwise we retain the existing result.
    pub fn handle_next_result(
        &mut self,
        stream: &mut Stream,
        next_result: ParserResult,
    ) -> ParserFlow {
        match &mut self.result {
            finished_state!() => return ParserFlow::Break(()),

            ParserResult::IncompleteMatch(ref mut running_result) => match next_result {
                ParserResult::Match(_) | ParserResult::PrattOperatorMatch(_) => {
                    self.result = next_result;
                }
                ParserResult::IncompleteMatch(ref next) => {
                    if next.byte_len() > running_result.byte_len() {
                        self.result = next_result;
                    }
                }

                ParserResult::NoMatch(_) => {}
            },

            ParserResult::NoMatch(ref mut running_result) => match next_result {
                ParserResult::Match(_)
                | ParserResult::PrattOperatorMatch(_)
                | ParserResult::IncompleteMatch(_) => {
                    self.result = next_result;
                }

                ParserResult::NoMatch(next_result) => running_result
                    .tokens_that_would_have_allowed_more_progress
                    .extend(next_result.tokens_that_would_have_allowed_more_progress),
            },
        }

        match self.result {
            finished_state!() => ParserFlow::Break(()),
            _ => {
                stream.set_position(self.start_position);
                ParserFlow::Continue(())
            }
        }
    }

    /// Executes a closure that allows the caller to drive the choice parse.
    ///
    /// Useful when you want to eagerly return a result from the parse function (e.g. when the choice was fully matched).
    ///
    /// Usage:
    /// ```no_run
    /// # use codegen_parser_runtime::support::{ParserResult, SequenceHelper};
    /// # fn parse_something() -> ParserResult { ParserResult::r#match(vec![], vec![]) }
    /// # fn parse_another() -> ParserResult { ParserResult::r#match(vec![], vec![]) }
    /// SequenceHelper::run(|mut sequence| {
    ///     sequence.elem(parse_something())?;
    ///     sequence.elem(parse_another())?;
    ///     sequence.finish()
    /// });
    /// ```
    pub fn run(
        stream: &mut Stream,
        f: impl FnOnce(Self, &mut Stream) -> ControlFlow<ParserResult, Self>,
    ) -> ParserResult {
        match f(ChoiceHelper::new(stream), stream) {
            ControlFlow::Break(result) => result,
            ControlFlow::Continue(helper) => helper.unwrap_result(stream),
        }
    }

    /// Aggregates a choice result into the accumulator. If we fully matched, returns the match.
    pub fn consider(
        &mut self,
        stream: &mut Stream,
        value: ParserResult,
    ) -> ControlFlow<ParserResult, &mut Self> {
        let result = self.handle_next_result(stream, value);
        match result {
            ParserFlow::Break(()) => ControlFlow::Break(self.take_result(stream)),
            ParserFlow::Continue(()) => ControlFlow::Continue(self),
        }
    }

    /// Finishes the choice parse, returning the accumulated match.
    pub fn finish(self, stream: &mut Stream) -> ControlFlow<ParserResult, Self> {
        ControlFlow::Break(self.unwrap_result(stream))
    }

    fn take_result(&mut self, stream: &mut Stream) -> ParserResult {
        if let ParserResult::IncompleteMatch(incomplete_match) = &self.result {
            incomplete_match.consume_stream(stream);
        }

        std::mem::replace(&mut self.result, ParserResult::no_match(vec![]))
    }

    fn unwrap_result(self, stream: &mut Stream) -> ParserResult {
        if let ParserResult::IncompleteMatch(incomplete_match) = &self.result {
            incomplete_match.consume_stream(stream);
        }
        self.result
    }
}
