use codegen_language_definition::model;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::parser::codegen::scanner_definition::ScannerExt as _;
use crate::parser::codegen::versioned::VersionedQuote;
use crate::parser::grammar::KeywordScannerDefinitionRef;

pub trait KeywordScannerDefinitionCodegen {
    fn to_scanner_code(&self) -> TokenStream;
}

impl KeywordScannerDefinitionCodegen for KeywordScannerDefinitionRef {
    fn to_scanner_code(&self) -> TokenStream {
        let name_ident = format_ident!("{}", self.name());
        let terminal_kind = quote! { TerminalKind::#name_ident };

        let kw_scanners: Vec<_> = self
            .definitions()
            .iter()
            .map(|versioned_kw| {
                let scanner = versioned_kw.value.to_scanner_code();
                let enabled_cond = versioned_kw.enabled.as_ref().as_bool_expr();
                let reserved_cond = versioned_kw.reserved.as_ref().as_bool_expr();

                // Simplify the emitted code if we trivially know that reserved or enabled is true
                match (&*reserved_cond.to_string(), &*enabled_cond.to_string()) {
                    ("true", _) => quote! {
                        if #scanner {
                            KeywordScan::Reserved(#terminal_kind)
                        } else {
                            KeywordScan::Absent
                        }
                    },
                    ("false", _) => quote! {
                        if #enabled_cond && #scanner {
                            KeywordScan::Present(#terminal_kind)
                        } else {
                            KeywordScan::Absent
                        }
                    },
                    (_, "true") => quote! {
                        if #scanner {
                            if #reserved_cond {
                                KeywordScan::Reserved(#terminal_kind)
                            } else {
                                KeywordScan::Present(#terminal_kind)
                            }
                        } else {
                            KeywordScan::Absent
                        }
                    },
                    (_, "false") => quote! {
                        if #reserved_cond && #scanner {
                            KeywordScan::Reserved(#terminal_kind)
                        } else {
                            KeywordScan::Absent
                        }
                    },
                    _ => quote! {
                        if (#reserved_cond || #enabled_cond) && #scanner {
                            if #reserved_cond {
                                KeywordScan::Reserved(#terminal_kind)
                            } else {
                                KeywordScan::Present(#terminal_kind)
                            }
                        } else {
                            KeywordScan::Absent
                        }
                    },
                }
            })
            .collect();

        match &kw_scanners[..] {
            [] => quote! { KeywordScan::Absent },
            multiple => quote! { scan_keyword_choice!(input, ident, #(#multiple),*) },
        }
    }
}

impl KeywordScannerDefinitionCodegen for model::KeywordValue {
    fn to_scanner_code(&self) -> TokenStream {
        // This is a subset; let's reuse that
        self.clone().into_scanner().to_scanner_code()
    }
}

trait IntoScanner {
    fn into_scanner(self) -> model::Scanner;
}

impl IntoScanner for model::KeywordValue {
    fn into_scanner(self: model::KeywordValue) -> model::Scanner {
        match self {
            model::KeywordValue::Optional { value } => model::Scanner::Optional {
                scanner: Box::new(value.into_scanner()),
            },
            model::KeywordValue::Sequence { values } => model::Scanner::Sequence {
                scanners: values.into_iter().map(IntoScanner::into_scanner).collect(),
            },
            model::KeywordValue::Atom { atom } => model::Scanner::Atom { atom },
            model::KeywordValue::Choice { values } => model::Scanner::Choice {
                scanners: values.into_iter().map(IntoScanner::into_scanner).collect(),
            },
        }
    }
}
