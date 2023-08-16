use crate::cst;
use crate::support::ParserResult;

use super::Stream;

impl ParserResult {
    pub fn try_recover_with(
        self,
        stream: &mut Stream,
        greedy_parse_until: impl Fn(&mut Stream) -> Vec<cst::Node>,
    ) -> ParserResult {
        match self {
            ParserResult::IncompleteMatch(mut result) => {
                let children = greedy_parse_until(stream);

                result.nodes.extend(children);

                // NOTE: We convert here to Match but this might be unexpected to the other combinators, since this
                // isn't really a full match. We need to check if we need to add some new variant (like `Recovered`)
                // or if we need to adapt the remaining combinators to handle this (e.g. wrt choice and span size ordering)
                ParserResult::r#match(result.nodes, result.expected_tokens)
            }
            result => result,
        }
    }
}
