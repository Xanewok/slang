// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use std::{ops::ControlFlow, rc::Rc};

use crate::{
    cst,
    support::{ParserFlow, ParserResult},
};

// The sequence is finished (can't make more progress) when we have an incomplete or no match.
macro_rules! finished_state {
    () => {
        ParserResult::IncompleteMatch(..) | ParserResult::NoMatch(..)
    };
}

/// Keeps accumulating parses until it hits an incomplete or no match.
#[must_use]
#[derive(Default)]
pub struct SequenceHelper {
    result: Option<ParserResult>,
}

impl SequenceHelper {
    pub fn is_done(&self) -> bool {
        matches!(self.result, Some(finished_state!()))
    }

    /// Accumulates a next result - tries to append it to the existing result until we hit an incomplete/no match.
    pub fn handle_next_result(&mut self, next_result: ParserResult) -> ParserFlow {
        match self.result {
            // Base case: we were just constructed, just take the next result
            None => self.result = Some(next_result),
            Some(ref mut result) => match (result, next_result) {
                // Can't proceed further, return what we have
                (finished_state!(), _) => return ParserFlow::Break(()),

                // If the accumulated result is valid, but empty (e.g. we accepted an empty optional)
                // just take the next result
                (ParserResult::Match(running), next @ _) if running.nodes.is_empty() => {
                    self.result = Some(next);
                }
                // Keep accepting or convert into PrattOperatorMatch
                (ParserResult::Match(running), ParserResult::Match(next)) => {
                    running.nodes.extend(next.nodes);
                    running.expected_tokens = next.expected_tokens;
                }
                (ParserResult::Match(running), ParserResult::PrattOperatorMatch(next)) => {
                    let mut children = vec![(0, std::mem::take(&mut running.nodes), 0)];
                    children.extend(next.nodes);
                    self.result = Some(ParserResult::pratt_operator_match(children));
                }
                // End of a valid sequence, finish with an incomplete match
                (ParserResult::Match(running), ParserResult::IncompleteMatch(next)) => {
                    running.nodes.extend(next.nodes);
                    self.result = Some(ParserResult::incomplete_match(
                        std::mem::take(&mut running.nodes),
                        next.expected_tokens,
                    ));
                }
                (ParserResult::Match(running), ParserResult::NoMatch(next)) => {
                    running.expected_tokens.extend(next.expected_tokens);

                    // Mark this point as incomplete
                    if let Some(err) = running.nodes.last_mut().and_then(|x| x.as_error_mut()) {
                        let inner =
                            Rc::get_mut(err).expect("Nothing to point to the inner error node");
                        inner.expected.extend(running.expected_tokens.clone());
                    } else {
                        running.nodes.push(cst::Node::error(
                            "".to_string(),
                            running.expected_tokens.clone(),
                        ));
                    }

                    self.result = Some(ParserResult::incomplete_match(
                        std::mem::take(&mut running.nodes),
                        std::mem::take(&mut running.expected_tokens),
                    ));
                }
                // Keep accepting or convert Match -> PrattOperatorMatch
                (ParserResult::PrattOperatorMatch(running), ParserResult::Match(next)) => {
                    if !next.nodes.is_empty() {
                        running.nodes.push((0, next.nodes, 0));
                    }
                }
                (ParserResult::PrattOperatorMatch(cur), ParserResult::PrattOperatorMatch(next)) => {
                    cur.nodes.extend(next.nodes);
                }
                // End of a valid sequence, finish with an incomplete match
                (ParserResult::PrattOperatorMatch(cur), ParserResult::IncompleteMatch(next)) => {
                    self.result = Some(ParserResult::incomplete_match(
                        std::mem::take(&mut cur.nodes)
                            .into_iter()
                            .flat_map(|(_, n, _)| n)
                            .chain(next.nodes.into_iter())
                            .collect(),
                        next.expected_tokens,
                    ));
                }
                (ParserResult::PrattOperatorMatch(cur), ParserResult::NoMatch(next)) => {
                    // Mark this point as incomplete
                    cur.nodes.push((
                        0,
                        vec![cst::Node::error(
                            "".to_string(),
                            next.expected_tokens.clone(),
                        )],
                        0,
                    ));

                    self.result = Some(ParserResult::incomplete_match(
                        std::mem::take(&mut cur.nodes)
                            .into_iter()
                            .flat_map(|(_, n, _)| n)
                            .collect(),
                        next.expected_tokens,
                    ));
                }
            },
        }

        // If we can't make any more progress, we have to stop
        // TODO(recovery): Handle partial parses?
        match self.result {
            Some(finished_state!()) => ParserFlow::Break(()),
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
