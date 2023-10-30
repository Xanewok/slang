use std::ops::ControlFlow;
use std::rc::Rc;

use anyhow::{anyhow, Error, Result};
use semver::Version;

use slang_solidity::{
    cst::{Node, RuleNode},
    cursor::Cursor,
    kinds::{ProductionKind, RuleKind, TokenKind},
    language::Language,
    visitor::{Step, Visitor},
};

struct ContractCollector {
    contract_names: Vec<String>,
}

impl Visitor<Error> for ContractCollector {
    fn rule_enter(
        &mut self,
        node: &Rc<RuleNode>,
        _cursor: &Cursor,
    ) -> ControlFlow<Result<(), Error>, Step> {
        if node.kind == RuleKind::ContractDefinition {
            match &node.children[2] {
                Node::Token(token) if token.kind == TokenKind::Identifier => {
                    self.contract_names.push(token.text.to_owned());
                }
                _ => {
                    return ControlFlow::Break(Err(anyhow!(
                        "Expected contract identifier: {node:?}"
                    )))
                }
            }

            return ControlFlow::Continue(Step::Over);
        }

        ControlFlow::Continue(Step::In)
    }
}

#[test]
fn visitor_api() -> Result<()> {
    let language = Language::new(Version::parse("0.8.0")?)?;
    let parse_output = language.parse(ProductionKind::ContractDefinition, "contract Foo {}");

    let mut collector = ContractCollector {
        contract_names: Vec::new(),
    };

    if let ControlFlow::Break(result) = parse_output
        .parse_tree()
        .cursor()
        .drive_visitor(&mut collector)
    {
        result?;
    }

    assert!(matches!(&collector.contract_names[..], [single] if single == "Foo"));

    return Ok(());
}
