use std::collections::BTreeSet;
use std::fmt::Debug;
use std::rc::Rc;

use codegen_language_definition::model::{self, Identifier};
use proc_macro2::TokenStream;

use crate::parser::grammar::{GrammarVisitor, Visitable};

pub trait ScannerDefinition: Debug {
    /// A unique identifier for this scanner.
    fn name(&self) -> &Identifier;
    /// Quotes the matching Rust scanner code.
    fn to_scanner_code(&self) -> TokenStream;
    /// A set of literals that this scanner can match.
    ///
    /// If the scanner matches more than just (a union of) literals, this method should return `None`.
    fn literals(&self) -> Option<BTreeSet<String>>;
    /// For which language version the scanner is defined.
    fn version_specifier(&self) -> Option<&model::VersionSpecifier> {
        None
    }
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
