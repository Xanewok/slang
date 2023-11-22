use crate::model::{Identifier, Scanner, Spanned, VersionSpecifier};
use codegen_language_internal_macros::{ParseInputTokens, WriteOutputTokens};
use serde::Serialize;

#[derive(Clone, Debug, Eq, ParseInputTokens, PartialEq, Serialize, WriteOutputTokens)]
pub struct TokenItem {
    pub name: Spanned<Identifier>,

    pub definitions: Vec<TokenDefinition>,
}

#[derive(Clone, Debug, Eq, ParseInputTokens, PartialEq, Serialize, WriteOutputTokens)]
pub struct TokenDefinition {
    pub enabled: Option<Spanned<VersionSpecifier>>,

    pub scanner: Scanner,
}
