use super::parser_result::{ParserResult, PrattElement};

pub struct SequenceHelper {
    result: ParserResult,
    is_done: bool,
}

impl SequenceHelper {
    pub fn new() -> Self {
        Self {
            result: ParserResult::no_match(vec![]),
            is_done: false,
        }
    }

    pub fn handle_next_result(&mut self, next_result: ParserResult) -> bool {
        if self.is_done {
            return true;
        }

        match &mut self.result {
            ParserResult::Match(running_result) if running_result.nodes.is_empty() => {
                self.result = next_result;
            }

            ParserResult::Match(ref mut running_result) => match next_result {
                ParserResult::Match(next_result) => {
                    running_result.nodes.extend(next_result.nodes);
                    running_result.tokens_that_would_have_allowed_more_progress =
                        next_result.tokens_that_would_have_allowed_more_progress;
                }

                ParserResult::PrattOperatorMatch(next_result) => {
                    let mut children = vec![PrattElement::Expression {
                        nodes: std::mem::take(&mut running_result.nodes),
                    }];
                    children.extend(next_result.elements);
                    self.result = ParserResult::pratt_operator_match(children);
                }

                ParserResult::IncompleteMatch(next_result) => {
                    running_result.nodes.extend(next_result.nodes);
                    self.result = ParserResult::incomplete_match(
                        std::mem::take(&mut running_result.nodes),
                        next_result.tokens_that_would_have_allowed_more_progress,
                    );
                }

                ParserResult::NoMatch(next_result) => {
                    running_result
                        .tokens_that_would_have_allowed_more_progress
                        .extend(next_result.tokens_that_would_have_allowed_more_progress);
                    self.result = ParserResult::incomplete_match(
                        std::mem::take(&mut running_result.nodes),
                        std::mem::take(
                            &mut running_result.tokens_that_would_have_allowed_more_progress,
                        ),
                    );
                }
            },

            ParserResult::PrattOperatorMatch(ref mut runnning_result) => match next_result {
                ParserResult::Match(next_result) => {
                    if !next_result.nodes.is_empty() {
                        runnning_result.elements.push(PrattElement::Expression {
                            nodes: next_result.nodes,
                        });
                    }
                }

                ParserResult::PrattOperatorMatch(next_result) => {
                    runnning_result.elements.extend(next_result.elements);
                }

                ParserResult::IncompleteMatch(next_result) => {
                    self.result = ParserResult::incomplete_match(
                        std::mem::take(&mut runnning_result.elements)
                            .into_iter()
                            // TODO: Surely we don't need to clone these
                            .map(|pratt_element| pratt_element.to_nodes())
                            .flatten()
                            .chain(next_result.nodes.into_iter())
                            .collect(),
                        next_result.tokens_that_would_have_allowed_more_progress,
                    );
                }

                ParserResult::NoMatch(next_result) => {
                    self.result = ParserResult::incomplete_match(
                        std::mem::take(&mut runnning_result.elements)
                            .into_iter()
                            // TODO: Surely we don't need to clone these
                            .map(|pratt_element| pratt_element.to_nodes())
                            .flatten()
                            .collect(),
                        next_result.tokens_that_would_have_allowed_more_progress,
                    );
                }
            },

            ParserResult::IncompleteMatch(_) => unreachable!("SequenceHelper is done"),

            ParserResult::NoMatch(_) => {
                self.result = next_result;
            }
        }

        self.is_done = !self.result.is_match();

        self.is_done
    }

    pub fn result(self) -> ParserResult {
        self.result
    }
}