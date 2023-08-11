// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use std::ops::ControlFlow;

use super::{ParserFlow, ParserResult};

// The sequence is finished (can't make more progress) when we have an incomplete or no match.
macro_rules! finished_state {
    () => {
        Some(ParserResult::IncompleteMatch(..) | ParserResult::NoMatch(..))
    };
}

#[must_use]
#[derive(Default)]
pub struct SequenceHelper {
    result: Option<ParserResult>,
}

impl SequenceHelper {
    pub fn is_done(&self) -> bool {
        matches!(self.result, finished_state!())
    }

    /// Accumulates a next result - tries to append it to the existing result until we hit an incomplete/no match.
    pub fn handle_next_result(&mut self, next_result: ParserResult) -> ParserFlow {
        match &mut self.result {
            finished_state!() => return ParserFlow::Break(()),
            // Base case - if we were just constructed, we just take the next result
            None => self.result = Some(next_result),

            // Similarly, if the accumulated result is valid, but empty (e.g. we accepted an empty optional)
            Some(ParserResult::Match(running_result)) if running_result.nodes.is_empty() => {
                self.result = Some(next_result);
            }

            // We fully matched at least one sequence element
            Some(ParserResult::Match(ref mut running_result)) => match next_result {
                ParserResult::Match(next_result) => {
                    running_result.nodes.extend(next_result.nodes);
                    running_result.tokens_that_would_have_allowed_more_progress =
                        next_result.tokens_that_would_have_allowed_more_progress;
                }

                // Combine the results and convert to Pratt operator match
                ParserResult::PrattOperatorMatch(next_result) => {
                    let mut children = vec![(0, std::mem::take(&mut running_result.nodes), 0)];
                    children.extend(next_result.nodes);
                    self.result = Some(ParserResult::pratt_operator_match(children));
                }
                // Combine the results but prepare to return an incomplete match
                ParserResult::IncompleteMatch(next_result) => {
                    running_result.nodes.extend(next_result.nodes);
                    self.result = Some(ParserResult::incomplete_match(
                        std::mem::take(&mut running_result.nodes),
                        next_result.tokens_that_would_have_allowed_more_progress,
                    ));
                }
                //
                ParserResult::NoMatch(next_result) => {
                    running_result
                        .tokens_that_would_have_allowed_more_progress
                        .extend(next_result.tokens_that_would_have_allowed_more_progress);
                    self.result = Some(ParserResult::incomplete_match(
                        std::mem::take(&mut running_result.nodes),
                        std::mem::take(
                            &mut running_result.tokens_that_would_have_allowed_more_progress,
                        ),
                    ));
                }
            },

            Some(ParserResult::PrattOperatorMatch(ref mut runnning_result)) => match next_result {
                ParserResult::Match(next_result) => {
                    if !next_result.nodes.is_empty() {
                        runnning_result.nodes.push((0, next_result.nodes, 0));
                    }
                }

                ParserResult::PrattOperatorMatch(next_result) => {
                    runnning_result.nodes.extend(next_result.nodes);
                }

                ParserResult::IncompleteMatch(next_result) => {
                    self.result = Some(ParserResult::incomplete_match(
                        std::mem::take(&mut runnning_result.nodes)
                            .into_iter()
                            .map(|(_, n, _)| n)
                            .flatten()
                            .chain(next_result.nodes.into_iter())
                            .collect(),
                        next_result.tokens_that_would_have_allowed_more_progress,
                    ));
                }

                ParserResult::NoMatch(next_result) => {
                    self.result = Some(ParserResult::incomplete_match(
                        std::mem::take(&mut runnning_result.nodes)
                            .into_iter()
                            .map(|(_, n, _)| n)
                            .flatten()
                            .collect(),
                        next_result.tokens_that_would_have_allowed_more_progress,
                    ));
                }
            },
        }

        // If we can't make any more progress, we have to stop
        // TODO(recovery): Handle partial parses?
        match self.result {
            finished_state!() => ParserFlow::Break(()),
            _ => ParserFlow::Continue(()),
        }
    }

    /// Executes a closure that allows the caller to drive the sequence parse.
    ///
    /// Useful when you want to eagerly return a result from the parse function (e.g. when we can't make more progress).
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
    pub fn run(f: impl FnOnce(Self) -> ControlFlow<ParserResult, Self>) -> ParserResult {
        match f(SequenceHelper::default()) {
            ControlFlow::Break(result) => result,
            ControlFlow::Continue(helper) => helper.unwrap_result(),
        }
    }

    /// Aggregates a parse result into the sequence. If we cannot make progress, returns the accumulated match.
    pub fn elem(&mut self, value: ParserResult) -> ControlFlow<ParserResult, &mut Self> {
        let result = self.handle_next_result(value);
        match result {
            ParserFlow::Break(()) => ControlFlow::Break(self.take_result()),
            ParserFlow::Continue(()) => ControlFlow::Continue(self),
        }
    }

    /// Finishes the sequence parse, returning the accumulated match.
    pub fn finish(self) -> ControlFlow<ParserResult, Self> {
        ControlFlow::Break(self.unwrap_result())
    }

    fn take_result(&mut self) -> ParserResult {
        std::mem::take(&mut self.result).expect("SequenceHelper was not driven")
    }

    fn unwrap_result(self) -> ParserResult {
        match self.result {
            Some(result) => result,
            None => panic!("SequenceHelper was not driven"),
        }
    }
}
