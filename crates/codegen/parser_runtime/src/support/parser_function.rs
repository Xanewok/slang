use std::{convert::Infallible, rc::Rc};

use super::{
    super::{
        cst,
        cursor::Cursor,
        parse_error::ParseError,
        parse_output::ParseOutput,
        visitor::{Visitor, VisitorExitResponse},
    },
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
            })
            | ParserResult::Match(Match {
                mut nodes,
                expected_tokens,
            }) => {
                if nodes.len() != 1 {
                    unreachable!("Match at the top level of a parser has more than one node")
                }
                if let cst::Node::Rule(rule_node) = nodes.remove(0) {
                    let outer = cst::Node::Rule(rule_node.clone());
                    let errors = collect_errors_with_ranges(&outer);

                    // The stream was not entirely consumed, mark the rest as skipped
                    let start = stream.position();
                    if start.utf8 < input.len() {
                        let unexpected = cst::Node::error(
                            input[start.utf8..].to_string(),
                            expected_tokens.clone(),
                        );
                        let mut new_children = rule_node.children.clone();
                        new_children.push(unexpected);
                        let mut errors = errors;
                        errors.push(ParseError::new_covering_range(
                            start..input.into(),
                            expected_tokens,
                        ));

                        ParseOutput {
                            parse_tree: cst::Node::rule(rule_node.kind, new_children),
                            errors,
                        }
                    } else {
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

fn collect_errors_with_ranges(node: &cst::Node) -> Vec<ParseError> {
    struct Errors(Vec<ParseError>);

    impl Visitor<Infallible> for Errors {
        fn error(
            &mut self,
            node: &Rc<cst::ErrorNode>,
            cursor: &Cursor,
        ) -> Result<VisitorExitResponse, Infallible> {
            self.0.push(ParseError::new_covering_range(
                cursor.text_range(),
                node.expected.clone(),
            ));
            Ok(VisitorExitResponse::Continue)
        }
    }

    let mut errors = Errors(vec![]);
    match node.cursor().drive_visitor(&mut errors) {
        Ok(_) => errors.0,
        Err(_) => unreachable!(),
    }
}
