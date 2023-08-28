use super::{
    parser_result::{IncompleteMatch, NoMatch, ParserResult},
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
        let mut accum = match parser(stream) {
            // First item parsed correctly
            result @ ParserResult::Match(_) => result,
            result @ ParserResult::PrattOperatorMatch(_) => result,

            // Couldn't get a full match but we allow 0 items - return an empty match
            // so the parse is considered valid but note the expected tokens
            ParserResult::IncompleteMatch(IncompleteMatch {
                tokens_that_would_have_allowed_more_progress,
                ..
            })
            | ParserResult::NoMatch(NoMatch {
                tokens_that_would_have_allowed_more_progress,
                ..
            }) if MIN_COUNT == 0 => {
                stream.set_position(save);
                return ParserResult::r#match(vec![], tokens_that_would_have_allowed_more_progress);
            }
            // Don't try repeating if we don't have a full match and we require at least one
            incomplete_or_no_match => return incomplete_or_no_match,
        };

        loop {
            let save = stream.position();
            let next_result = parser(stream);

            match (&mut accum, next_result) {
                (ParserResult::Match(running), ParserResult::Match(next)) => {
                    running.nodes.extend(next.nodes);
                    running.tokens_that_would_have_allowed_more_progress =
                        next.tokens_that_would_have_allowed_more_progress;
                }

                (ParserResult::PrattOperatorMatch(cur), ParserResult::PrattOperatorMatch(next)) => {
                    cur.elements.extend(next.elements);
                }

                (ParserResult::Match(..), ParserResult::PrattOperatorMatch(..)) => unreachable!(
                    "PrattOperatorMatch seen while repeating Matches in RepetitionHelper"
                ),
                (ParserResult::PrattOperatorMatch(..), ParserResult::Match(..)) => unreachable!(
                    "Match seen while repeating PrattOperatorMatches in RepetitionHelper"
                ),
                // Can't proceed further with a complete parse, so back up, return
                // the accumulated result and note the expected tokens
                (
                    ParserResult::Match(running),
                    ParserResult::IncompleteMatch(IncompleteMatch {
                        tokens_that_would_have_allowed_more_progress,
                        ..
                    })
                    | ParserResult::NoMatch(NoMatch {
                        tokens_that_would_have_allowed_more_progress,
                    }),
                ) => {
                    stream.set_position(save);
                    running.tokens_that_would_have_allowed_more_progress =
                        tokens_that_would_have_allowed_more_progress;
                    return accum;
                }

                (
                    ParserResult::PrattOperatorMatch(_),
                    ParserResult::IncompleteMatch(_) | ParserResult::NoMatch(_),
                ) => {
                    stream.set_position(save);
                    return accum;
                }

                (ParserResult::IncompleteMatch(..) | ParserResult::NoMatch(..), _) => {
                    unreachable!("Variants never constructed")
                }
            }
        }
    }
}
