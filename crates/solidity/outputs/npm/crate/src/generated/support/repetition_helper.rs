// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use super::{
    parser_result::{IncompleteMatch, Match, NoMatch, ParserResult},
    stream::Stream,
};

pub struct RepetitionHelper<const MIN_COUNT: usize>;

pub type ZeroOrMoreHelper = RepetitionHelper<0>;
pub type OneOrMoreHelper = RepetitionHelper<1>;

impl<const MIN_COUNT: usize> RepetitionHelper<MIN_COUNT> {
    pub fn run<F: Fn(&mut Stream) -> ParserResult>(stream: &mut Stream, parser: F) -> ParserResult {
        if MIN_COUNT > 1 {
            unimplemented!("RepetitionHelper only supports min_count of 0 or 1")
        }

        let save = stream.position();
        let mut result = parser(stream);

        match result {
            // First item parsed correctly
            ParserResult::Match(_) => {}
            ParserResult::PrattOperatorMatch(_) => {}

            // Couldn't get a full match but we allow 0 items - return an empty match
            // so the parse is considered valid but note the expected tokens
            ParserResult::IncompleteMatch(IncompleteMatch {
                expected_tokens, ..
            })
            | ParserResult::NoMatch(NoMatch {
                expected_tokens, ..
            }) if MIN_COUNT == 0 => {
                stream.set_position(save);
                return ParserResult::r#match(vec![], expected_tokens);
            }
            // Don't try repeating if we don't have a full match and we require at least one
            // TODO(recovery): We should allow for incomplete matches in recovery
            ParserResult::IncompleteMatch(..) | ParserResult::NoMatch(..) => return result,
        }

        loop {
            let save = stream.position();
            let next_result = parser(stream);

            match result {
                ParserResult::Match(ref mut current_match) => match next_result {
                    ParserResult::Match(Match {
                        nodes,
                        expected_tokens,
                    }) => {
                        current_match.nodes.extend(nodes);
                        current_match.expected_tokens = expected_tokens;
                    }

                    ParserResult::PrattOperatorMatch(_) => unreachable!(
                        "PrattOperatorMatch seen while repeating Matches in RepetitionHelper"
                    ),
                    // Can't proceed further with a complete parse, so back up, return
                    // the accumulated result and note the expected tokens
                    // TODO(recovery): We should allow for incomplete matches
                    ParserResult::IncompleteMatch(IncompleteMatch {
                        expected_tokens, ..
                    })
                    | ParserResult::NoMatch(NoMatch { expected_tokens }) => {
                        stream.set_position(save);
                        current_match.expected_tokens = expected_tokens;
                        return result;
                    }
                },

                ParserResult::PrattOperatorMatch(ref mut current_match) => {
                    match next_result {
                        ParserResult::Match(_) => unreachable!(
                            "Match seen while repeating PrattOperatorMatches in RepetitionHelper"
                        ),

                        ParserResult::PrattOperatorMatch(r#match) => {
                            current_match.nodes.extend(r#match.nodes);
                        }

                        ParserResult::IncompleteMatch(_) | ParserResult::NoMatch(_) => {
                            stream.set_position(save);
                            return result;
                        }
                    };
                }

                ParserResult::IncompleteMatch(_) => {
                    unreachable!("IncompleteMatch is never constructed")
                }

                ParserResult::NoMatch(_) => {
                    unreachable!("NoMatch is never constructed")
                }
            }
        }
    }
}
