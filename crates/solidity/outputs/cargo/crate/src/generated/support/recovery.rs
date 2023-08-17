// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use std::ops::Range;
use std::rc::Rc;

use crate::cst;
use crate::support::ParserResult;

use super::Stream;

impl ParserResult {
    pub fn try_recover_with(
        self,
        stream: &mut Stream,
        skip_tokens_for_recovery: impl Fn(&mut Stream) -> Option<Range<usize>>,
    ) -> ParserResult {
        match self {
            ParserResult::IncompleteMatch(mut result) => {
                fn last_error(node: &mut Vec<cst::Node>) -> Option<&mut cst::ErrorNode> {
                    node.last_mut()
                        .and_then(cst::Node::as_error_mut)
                        .and_then(Rc::get_mut)
                }

                let skipped = skip_tokens_for_recovery(stream);
                // Extend the last error if exists or create a new node
                if let Some(skipped_content) = skipped.map(|range| stream.content(range)) {
                    if let Some(node) = last_error(&mut result.nodes) {
                        node.text += &skipped_content;
                    } else {
                        result.nodes.push(cst::Node::error(skipped_content, vec![]));
                    }
                }

                // NOTE: We convert here to Match but this might be unexpected to the other combinators, since this
                // isn't really a full match. We need to check if we need to add some new variant (like `Recovered`)
                // or if we need to adapt the remaining combinators to handle this (e.g. wrt choice and span size ordering)
                ParserResult::r#match(result.nodes, result.expected_tokens)
            }
            result => result,
        }
    }
}
