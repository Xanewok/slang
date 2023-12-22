use std::collections::BTreeMap;
use std::fmt::Debug;

use codegen_grammar::{ScannerDefinitionNode, ScannerDefinitionRef, VersionQualityRange};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::parser_definition::VersionQualityRangeVecExtensions;

#[derive(Clone, Debug, Default)]
pub struct Trie {
    pub subtries: BTreeMap<char, Self>,
    pub key: Option<String>,
    // pub payload: Option<ScannerDefinitionRef>,
    pub payload: Option<(String, Vec<VersionQualityRange>, Vec<VersionQualityRange>)>,
}

impl Trie {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(
        &mut self,
        key: String,
        token_name: String,
        versions: Vec<VersionQualityRange>,
        reserved: Vec<VersionQualityRange>,
    ) {
        let mut node = self;
        for char in key.chars() {
            node = node.subtries.entry(char).or_insert_with(Self::new);
        }
        node.payload = Some((token_name, versions, reserved));
        node.key = Some(key);
    }

    /// Finds the next node that has either a payload or more than one subtrie.
    /// It returns the path to that node and the node itself.
    fn next_interesting_node(&self, prefix: Option<char>) -> (Vec<char>, &Trie) {
        let mut path = prefix.map(|c| vec![c]).unwrap_or_default();
        let mut node = self;
        while node.payload.is_none() && node.subtries.len() == 1 {
            let (key, subtrie) = node.subtries.iter().next().unwrap();
            path.push(*key);
            node = subtrie;
        }
        (path, node)
    }

    pub fn to_scanner_code(&self) -> TokenStream {
        let (path, trie) = self.next_interesting_node(None);

        let branches = trie
            .subtries
            .iter()
            .map(|(c, subtrie)| {
                let child_code = subtrie.to_scanner_code();
                quote! { Some(#c) => #child_code }
            })
            .collect::<Vec<_>>();

        let leaf = if let Some((token_name, versions, reserved)) = &trie.payload {
            // let kind = format_ident!("{}", scanner_definition_ref.name());
            let kind = format_ident!("{}", token_name);

            if branches.is_empty() && !path.is_empty() {
                // This is an optimisation for a common case
                let leaf = quote! { scan_chars!(input, #(#path),*).then_some(TokenKind::#kind) };

                return wrap_enabled_and_reserved(leaf, versions, reserved);
            }

            wrap_enabled_and_reserved(quote! { Some(TokenKind::#kind) }, versions, reserved)
            // scanner_definition_ref
            // .node()
            // .enabled_version_ranges()
            // versions.wrap_code(quote! { Some(TokenKind::#kind) }, Some(quote! { None }))
        } else {
            quote! { None }
        };

        let trie_code = if branches.is_empty() {
            leaf
        } else {
            quote! {
                match input.next() {
                    #(#branches,)*
                    Some(_) => { input.undo(); #leaf }
                    None => #leaf,
                }
            }
        };

        if path.is_empty() {
            trie_code
        } else {
            quote! {
                if scan_chars!(input, #(#path),*).matched() {
                    #trie_code
                } else {
                    None
                }
            }
        }
    }
}

pub trait VersionWrapped {
    /// Returns a range when a scanner is enabled but ambiguous
    fn enabled_version_ranges(&self) -> Vec<VersionQualityRange>;
    /// Returns a range when a scanner is strict
    fn strict_version_ranges(&self) -> Vec<VersionQualityRange>;
}

impl VersionWrapped for ScannerDefinitionNode {
    fn enabled_version_ranges(&self) -> Vec<VersionQualityRange> {
        match self {
            ScannerDefinitionNode::Versioned(_, enabled, _) => enabled.clone(),

            ScannerDefinitionNode::Optional(node)
            | ScannerDefinitionNode::ZeroOrMore(node)
            | ScannerDefinitionNode::OneOrMore(node) => node.enabled_version_ranges(),

            _ => vec![],
        }
    }

    fn strict_version_ranges(&self) -> Vec<VersionQualityRange> {
        match self {
            ScannerDefinitionNode::Versioned(_, _, reserved) => reserved.clone(),

            ScannerDefinitionNode::Optional(node)
            | ScannerDefinitionNode::ZeroOrMore(node)
            | ScannerDefinitionNode::OneOrMore(node) => node.strict_version_ranges(),

            _ => vec![],
        }
    }
}

fn wrap_enabled_and_reserved(
    leaf: TokenStream,
    enabled: &Vec<VersionQualityRange>,
    reserved: &Vec<VersionQualityRange>,
) -> TokenStream {
    let leaf = leaf;
    let enabled = enabled.wrap_code(quote!(true), Some(quote!(false)));
    let reserved = reserved.wrap_code(quote!(true), Some(quote!(false)));

    quote! {
        {
            let enabled = #enabled;
            let reserved = #reserved;
        if enabled && reserved {
            // Strict keyword
            #leaf.map(|x| (x, false))
        } else if enabled {
            // Contextual keyword
            #leaf.map(|x| (x, true))
        } else if reserved {
            // Reserved word; we have to inform the lexer that we can't accept regular identifiers later
            #leaf.map(|x| (x, false))
        } else {
            // Not enabled nor reserved; we can't accept this
            None
        }
        }
    }
}
