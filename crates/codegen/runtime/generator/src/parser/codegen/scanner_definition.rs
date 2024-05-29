use std::collections::BTreeSet;
use std::ops::Deref;

use codegen_language_definition::model;
use inflector::Inflector;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::parser::codegen::versioned::VersionedQuote;
use crate::parser::grammar::ScannerDefinitionRef;

pub trait ScannerDefinitionCodegen {
    fn to_scanner_code(&self) -> TokenStream;
    /// Returns the literals of the scanner, if the is not compound.
    fn literals(&self) -> Vec<String>;
}

impl ScannerDefinitionCodegen for ScannerDefinitionRef {
    fn to_scanner_code(&self) -> TokenStream {
        self.deref().to_scanner_code()
    }

    fn literals(&self) -> Vec<String> {
        let mut result = BTreeSet::new();
        if self.deref().literals(&mut result) {
            result.into_iter().collect()
        } else {
            vec![]
        }
    }
}

pub trait ScannerDefinitionNodeCodegen {
    fn to_scanner_code(&self) -> TokenStream;
    /// Whether the scanner is an atom, and if so, returns the atom.
    fn as_atom(&self) -> Option<&str>;
    /// Keeps accumulating literals and returns whether the scanner is a literal.
    ///
    /// Short-circuits on the first non-literal scanner.
    #[doc(hidden)]
    fn literals_accum(&self, accum: &mut BTreeSet<String>) -> bool;
}

impl ScannerDefinitionCodegen for model::TriviaItem {
    fn to_scanner_code(&self) -> TokenStream {
        self.scanner.to_scanner_code()
    }

    fn literals(&self) -> Vec<String> {
        let mut result = BTreeSet::new();
        if self.scanner.literals_accum(&mut result) {
            result.into_iter().collect()
        } else {
            vec![]
        }
    }
}

impl ScannerDefinitionCodegen for model::FragmentItem {
    fn to_scanner_code(&self) -> TokenStream {
        VersionedScanner::new(&self.scanner, self.enabled.as_ref()).to_scanner_code()
    }

    fn literals(&self) -> Vec<String> {
        let mut result = BTreeSet::new();
        if self.scanner.literals_accum(&mut result) {
            result.into_iter().collect()
        } else {
            vec![]
        }
    }
}

impl ScannerDefinitionCodegen for model::TokenItem {
    fn to_scanner_code(&self) -> TokenStream {
        ScannerDefinitionNodeCodegen::to_scanner_code(self)
    }

    fn literals(&self) -> Vec<String> {
        let mut result = BTreeSet::new();
        if ScannerDefinitionNodeCodegen::literals_accum(self, &mut result) {
            result.into_iter().collect()
        } else {
            vec![]
        }
    }
}

impl ScannerDefinitionNodeCodegen for model::FragmentItem {
    fn to_scanner_code(&self) -> TokenStream {
        VersionedScanner::new(&self.scanner, self.enabled.as_ref()).to_scanner_code()
    }

    fn as_atom(&self) -> Option<&str> {
        None
    }

    fn literals_accum(&self, accum: &mut BTreeSet<String>) -> bool {
        self.scanner.literals_accum(accum)
    }
}

pub(crate) struct VersionedScanner<'a> {
    scanner: &'a model::Scanner,
    enabled: Option<&'a model::VersionSpecifier>,
}

impl ScannerDefinitionNodeCodegen for VersionedScanner<'_> {
    fn to_scanner_code(&self) -> TokenStream {
        let scanner = self.scanner.to_scanner_code();
        self.enabled
            .to_conditional_code(scanner, Some(quote! { false }))
    }

    fn as_atom(&self) -> Option<&str> {
        None
    }

    fn literals_accum(&self, accum: &mut BTreeSet<String>) -> bool {
        self.scanner.literals_accum(accum)
    }
}

impl<'a> VersionedScanner<'a> {
    fn new(scanner: &'a model::Scanner, enabled: Option<&'a model::VersionSpecifier>) -> Self {
        Self { scanner, enabled }
    }
}

impl ScannerDefinitionNodeCodegen for model::TriviaItem {
    fn to_scanner_code(&self) -> TokenStream {
        self.scanner.to_scanner_code()
    }

    fn as_atom(&self) -> Option<&str> {
        None
    }

    fn literals_accum(&self, accum: &mut BTreeSet<String>) -> bool {
        self.scanner.literals_accum(accum)
    }
}

impl ScannerDefinitionNodeCodegen for model::TokenItem {
    fn to_scanner_code(&self) -> TokenStream {
        let defs: Vec<_> = self
            .definitions
            .iter()
            .map(|def| VersionedScanner::new(&def.scanner, def.enabled.as_ref()))
            .collect();

        match defs.len() {
            0 => panic!("Token {} has no definitions", self.name),
            1 => defs.into_iter().next().unwrap().to_scanner_code(),
            _ => choice_to_scanner_code(&defs),
        }
    }

    fn as_atom(&self) -> Option<&str> {
        None
    }

    fn literals_accum(&self, accum: &mut BTreeSet<String>) -> bool {
        // TODO: Make sure this is correct
        let mut result = BTreeSet::new();
        if self
            .definitions
            .iter()
            .all(|def| def.scanner.literals_accum(&mut result))
        {
            accum.extend(result);
            true
        } else {
            false
        }
    }
}

fn choice_to_scanner_code<T: ScannerDefinitionNodeCodegen>(nodes: &[T]) -> TokenStream {
    let mut scanners = vec![];
    let mut non_literal_scanners = vec![];
    for node in nodes {
        if let Some(atom) = node.as_atom() {
            scanners.push(atom);
        } else {
            non_literal_scanners.push(node.to_scanner_code());
        }
    }
    scanners.sort_unstable();
    let mut scanners = scanners
        .iter()
        // We want the longest literals first, so we prefer the longest match
        .rev()
        .map(|string| {
            let chars = string.chars();
            quote! { scan_chars!(input, #(#chars),*) }
        })
        .collect::<Vec<_>>();
    scanners.extend(non_literal_scanners);
    quote! { scan_choice!(input, #(#scanners),*) }
}

impl ScannerDefinitionNodeCodegen for model::Scanner {
    fn to_scanner_code(&self) -> TokenStream {
        match self {
            model::Scanner::Optional { scanner } => {
                let scanner = scanner.to_scanner_code();
                quote! { scan_optional!(input, #scanner) }
            }
            model::Scanner::ZeroOrMore { scanner } => {
                let scanner = scanner.to_scanner_code();
                quote! { scan_zero_or_more!(input, #scanner) }
            }

            model::Scanner::OneOrMore { scanner } => {
                let scanner = scanner.to_scanner_code();
                quote! { scan_one_or_more!(input, #scanner) }
            }
            model::Scanner::Not { chars } => {
                let chars = chars.iter();
                quote! { scan_none_of!(input, #(#chars),*) }
            }
            model::Scanner::TrailingContext {
                scanner: node,
                not_followed_by: lookahead,
            } => {
                let scanner = node.to_scanner_code();
                let negative_lookahead_scanner = lookahead.to_scanner_code();
                quote! { scan_not_followed_by!(input, #scanner, #negative_lookahead_scanner) }
            }
            model::Scanner::Sequence { scanners } => {
                let scanners = scanners
                    .iter()
                    .map(|e| e.to_scanner_code())
                    .collect::<Vec<_>>();
                quote! { scan_sequence!(#(#scanners),*) }
            }
            model::Scanner::Choice { scanners: nodes } => choice_to_scanner_code(nodes),

            model::Scanner::Range {
                inclusive_start: from,
                inclusive_end: to,
            } => {
                quote! { scan_char_range!(input, #from..=#to) }
            }
            model::Scanner::Atom { atom } => {
                let chars = atom.chars();
                quote! { scan_chars!(input, #(#chars),*) }
            }

            model::Scanner::Fragment { reference } => {
                let snake_case = reference.to_snake_case();
                let scanner_function_name = format_ident!("{snake_case}");
                quote! { self.#scanner_function_name(input) }
            }
        }
    }

    fn as_atom(&self) -> Option<&str> {
        match self {
            model::Scanner::Atom { atom } => Some(atom),
            _ => None,
        }
    }

    fn literals_accum(&self, accum: &mut BTreeSet<String>) -> bool {
        match self {
            Self::Atom { atom } => {
                accum.insert(atom.clone());
                true
            }
            Self::Choice { scanners } => scanners
                .iter()
                .fold(true, |result, node| node.literals_accum(accum) && result),
            _ => false,
        }
    }
}
