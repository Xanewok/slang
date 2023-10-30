use std::ops::ControlFlow;
use std::rc::Rc;

use super::{cst::*, cursor::Cursor};

/// A Visitor pattern for traversing the CST.
///
/// The trait supports fallible iteration, i.e. the visitor can early return an error from the visit.
pub trait Visitor<E> {
    /// Called when the [`Visitor`] enters a [`RuleNode`].
    fn rule_enter(
        &mut self,
        _node: &Rc<RuleNode>,
        _cursor: &Cursor,
    ) -> ControlFlow<Result<(), E>, Step> {
        ControlFlow::Continue(Step::In)
    }

    /// Called when the [`Visitor`] exits a [`RuleNode`].
    fn rule_exit(
        &mut self,
        _node: &Rc<RuleNode>,
        _cursor: &Cursor,
    ) -> ControlFlow<Result<(), E>, ()> {
        ControlFlow::Continue(())
    }

    /// Called when the [`Visitor`] enters a [`TokenNode`].
    fn token(&mut self, _node: &Rc<TokenNode>, _cursor: &Cursor) -> ControlFlow<Result<(), E>, ()> {
        ControlFlow::Continue(())
    }
}

/// Whether the [`Visitor`] should should enter the children of a [`RuleNode`] or not.
pub enum Step {
    In,
    Over,
}

impl Cursor {
    pub fn drive_visitor<E, V: Visitor<E>>(
        &mut self,
        visitor: &mut V,
    ) -> ControlFlow<Result<(), E>, Step> {
        if self.is_completed() {
            return ControlFlow::Break(Ok(()));
        }

        loop {
            // Node clone is cheap because it's just an enum around an Rc
            match self.node() {
                Node::Rule(node) => {
                    match visitor.rule_enter(&node, self)? {
                        Step::Over => {}
                        Step::In => {
                            if self.go_to_first_child() {
                                self.drive_visitor(visitor)?;
                                self.go_to_parent();
                            }
                        }
                    }
                    visitor.rule_exit(&node, self)?;
                }

                Node::Token(node) => visitor.token(&node, self)?,
            }

            if !self.go_to_next_sibling() {
                return ControlFlow::Break(Ok(()));
            }
        }
    }
}
