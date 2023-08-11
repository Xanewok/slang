// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use super::{super::text_index::TextIndex, ParserFlow, ParserResult, Stream};

// The choice is finished when we have a full match.
macro_rules! finished_state {
    () => {
        ParserResult::Match(..) | ParserResult::PrattOperatorMatch(..)
    };
}

/// Starting from a given position in the stream, this helper will try to pick (and remember) a best match. Settles on
/// a full match if possible, otherwise on the best incomplete match.
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

    pub fn result(self, stream: &mut Stream) -> ParserResult {
        if let ParserResult::IncompleteMatch(incomplete_match) = &self.result {
            incomplete_match.consume_stream(stream);
        }
        self.result
    }
}
