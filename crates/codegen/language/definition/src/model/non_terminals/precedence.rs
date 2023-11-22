use crate::model::{Field, FieldsErrorRecovery, Identifier, Spanned, VersionSpecifier};
use codegen_language_internal_macros::{ParseInputTokens, WriteOutputTokens};
use indexmap::IndexMap;
use serde::Serialize;

#[derive(Clone, Debug, Eq, ParseInputTokens, PartialEq, Serialize, WriteOutputTokens)]
pub struct PrecedenceItem {
    pub name: Spanned<Identifier>,

    pub enabled: Option<Spanned<VersionSpecifier>>,

    pub precedence_expressions: Vec<PrecedenceExpression>,
    pub primary_expressions: Vec<PrimaryExpression>,
}

#[derive(Clone, Debug, Eq, ParseInputTokens, PartialEq, Serialize, WriteOutputTokens)]
pub struct PrecedenceExpression {
    pub name: Spanned<Identifier>,
    pub rule_name: Spanned<Identifier>,

    pub operators: Vec<PrecedenceOperator>,
}

#[derive(Clone, Debug, Eq, ParseInputTokens, PartialEq, Serialize, WriteOutputTokens)]
pub struct PrecedenceOperator {
    pub model: Spanned<OperatorModel>,

    pub enabled: Option<Spanned<VersionSpecifier>>,

    pub error_recovery: Option<FieldsErrorRecovery>,
    pub fields: IndexMap<Spanned<Identifier>, Field>,
}

#[derive(Clone, Debug, Eq, ParseInputTokens, PartialEq, Serialize, WriteOutputTokens)]
pub enum OperatorModel {
    Prefix,
    Postfix,
    BinaryLeftAssociative,
    BinaryRightAssociative,
}

#[derive(Clone, Debug, Eq, ParseInputTokens, PartialEq, Serialize, WriteOutputTokens)]
pub struct PrimaryExpression {
    pub expression: Spanned<Identifier>,

    pub enabled: Option<Spanned<VersionSpecifier>>,
}
