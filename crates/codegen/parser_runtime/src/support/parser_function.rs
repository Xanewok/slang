use super::{
    super::{cst, parse_error::ParseError, parse_output::ParseOutput},
    parser_result::*,
    stream::Stream,
};

// Return type of the function has to be a type parameter of the trait
pub trait ParserFunction<L, R>
where
    Self: Fn(&L, &mut Stream) -> R,
{
    fn parse(&self, language: &L, input: &str) -> ParseOutput;
}

impl<L, F> ParserFunction<L, ParserResult> for F
where
    F: Fn(&L, &mut Stream) -> ParserResult,
{
    fn parse(&self, language: &L, input: &str) -> ParseOutput {
        let mut stream = Stream::new(input);
        let start = stream.position();
        match self(language, &mut stream) {
            ParserResult::NoMatch(no_match) => ParseOutput {
                parse_tree: cst::Node::error(input.to_string(), no_match.expected_tokens.clone()),
                errors: vec![ParseError::new_covering_range(
                    Default::default()..input.into(),
                    no_match.expected_tokens,
                )],
            },
            ParserResult::IncompleteMatch(IncompleteMatch {
                mut nodes,
                expected_tokens,
            }) => {
                if nodes.len() != 1 {
                    unreachable!(
                        "IncompleteMatch at the top level of a parser has more than one node"
                    )
                }
                if let cst::Node::Rule(rule_node) = nodes.remove(0) {
                    let start = stream.position();
                    let unexpected =
                        cst::Node::error(input[start.utf8..].to_string(), expected_tokens.clone());

                    let mut new_children = rule_node.children.clone();
                    new_children.push(unexpected);
                    ParseOutput {
                        parse_tree: cst::Node::rule(rule_node.kind, new_children),
                        errors: vec![ParseError::new_covering_range(
                            start..input.into(),
                            expected_tokens,
                        )],
                    }
                } else {
                    unreachable!("IncompleteMatch at the top level of a parser is not a Rule node")
                }
            }
            ParserResult::Match(mut r#match) => {
                if r#match.nodes.len() != 1 {
                    unreachable!("Match at the top level of a parser has more than one node")
                }
                if let cst::Node::Rule(rule_node) = r#match.nodes.remove(0) {
                    // The stream was not entirely consumed, mark the rest as skipped
                    let cur = stream.position();
                    if cur.utf8 < input.len() {
                        let unexpected = cst::Node::error(
                            input[cur.utf8..].to_string(),
                            r#match.expected_tokens.clone(),
                        );
                        let mut new_children = rule_node.children.clone();
                        new_children.push(unexpected);

                        ParseOutput {
                            parse_tree: cst::Node::rule(rule_node.kind, new_children),
                            errors: vec![ParseError::new_covering_range(
                                cur..input.into(),
                                r#match.expected_tokens,
                            )],
                        }
                    } else {
                        // We skip the tokens as part of the error recovery. Make sure to report these as errors
                        let errors = rule_node
                            .skipped_tokens()
                            .map(|_token| {
                                ParseError::new_covering_range(
                                    // TODO: Keep track of and use the range of the token
                                    start..stream.position(),
                                    r#match.expected_tokens.clone(),
                                )
                            })
                            .collect();

                        ParseOutput {
                            parse_tree: cst::Node::Rule(rule_node),
                            errors,
                        }
                    }
                } else {
                    unreachable!("Match at the top level of a parser is not a Rule node")
                }
            }
            ParserResult::PrattOperatorMatch(..) => unreachable!("PrattOperatorMatch is internal"),
        }
    }
}
