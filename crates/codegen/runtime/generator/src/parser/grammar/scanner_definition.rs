use std::collections::BTreeSet;
use std::fmt::Debug;
use std::rc::Rc;

use codegen_language_definition::model::{self, Identifier};
use proc_macro2::TokenStream;

use crate::parser::grammar::{GrammarVisitor, Visitable};

pub trait ScannerDefinition: Debug {
    fn name(&self) -> &Identifier;
    // ----
    // Instead of def
    fn to_scanner_code(&self) -> TokenStream;
    fn literals(&self, accum: &mut BTreeSet<String>) -> bool;
    fn version_specifier(&self) -> Option<&model::VersionSpecifier>;
}

pub type ScannerDefinitionRef = Rc<dyn ScannerDefinition>;

impl Visitable for ScannerDefinitionRef {
    fn accept_visitor<V: GrammarVisitor>(&self, visitor: &mut V) {
        visitor.scanner_definition_enter(self);
    }
}

pub trait KeywordScannerDefinition: Debug {
    fn name(&self) -> &Identifier;
    fn identifier_scanner(&self) -> &Identifier;
    fn definitions(&self) -> &[model::KeywordDefinition];
}

pub type KeywordScannerDefinitionRef = Rc<dyn KeywordScannerDefinition>;

/// A [`KeywordScannerDefinitionRef`] that only has a single atom value.
///
/// The main usage for this type is to construct a keyword trie in parser generator, as trie will
/// only work with single atom values and keyword promotion needs to additionally account for
/// keyword reservation, rather than just literal presence.
#[derive(Clone)]
pub struct KeywordScannerAtomic(KeywordScannerDefinitionRef);

impl KeywordScannerAtomic {
    /// Wraps the keyword scanner definition if it is a single atom value.
    pub fn try_from_def(def: &KeywordScannerDefinitionRef) -> Option<Self> {
        match def.definitions() {
            [model::KeywordDefinition {
                value: model::KeywordValue::Atom { .. },
                ..
            }] => Some(Self(Rc::clone(def))),
            _ => None,
        }
    }
}

impl std::ops::Deref for KeywordScannerAtomic {
    type Target = KeywordScannerDefinitionRef;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl KeywordScannerAtomic {
    pub fn definition(&self) -> &model::KeywordDefinition {
        self.0
            .definitions()
            .first()
            .expect("KeywordScannerAtomic should have exactly one definition")
    }

    pub fn value(&self) -> &str {
        match self.definition() {
            model::KeywordDefinition {
                value: model::KeywordValue::Atom { atom },
                ..
            } => atom,
            _ => unreachable!("KeywordScannerAtomic should have a single atom value"),
        }
    }
}
