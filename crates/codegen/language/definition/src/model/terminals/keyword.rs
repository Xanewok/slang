use codegen_language_internal_macros::{derive_spanned_type, ParseInputTokens, WriteOutputTokens};
use itertools::Itertools;
use serde::{Deserialize, Serialize};

use crate::model::{Identifier, VersionSpecifier};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[derive_spanned_type(ParseInputTokens, WriteOutputTokens)]
pub struct KeywordItem {
    pub name: Identifier,
    pub identifier: Identifier,

    pub definitions: Vec<KeywordDefinition>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[derive_spanned_type(ParseInputTokens, WriteOutputTokens)]
pub struct KeywordDefinition {
    pub enabled: Option<VersionSpecifier>,
    pub reserved: Option<VersionSpecifier>,

    pub value: KeywordValue,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[derive_spanned_type(ParseInputTokens, WriteOutputTokens)]
pub enum KeywordValue {
    Sequence { values: Vec<KeywordValue> },
    Optional { value: Box<KeywordValue> },
    Choice { values: Vec<KeywordValue> },
    Atom { atom: String },
}

impl KeywordValue {
    /// Collects all possible variations generated by this value.
    pub fn collect_variations(&self) -> Vec<String> {
        match self {
            KeywordValue::Atom { atom } => vec![atom.to_owned()],
            KeywordValue::Optional { value } => {
                let mut results = value.collect_variations();
                results.insert(0, String::new());
                results
            }
            KeywordValue::Choice { values } => {
                values.iter().flat_map(Self::collect_variations).collect()
            }

            KeywordValue::Sequence { values } => {
                let matrix = values.iter().map(Self::collect_variations).collect_vec();

                // M x N x O...
                let results_len = matrix.iter().map(Vec::len).product();
                let mut results = vec![String::new(); results_len];

                let mut span = results_len;
                // Process M x (N x O...)
                for variations in matrix {
                    span /= variations.len();

                    for (i, result) in results.iter_mut().enumerate() {
                        result.push_str(&variations[i / span % variations.len()]);
                    }
                }

                results
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_atom() {
        let value = KeywordValue::Atom { atom: "foo".into() };

        assert_eq!(value.collect_variations(), vec!["foo"]);
    }

    #[test]
    fn test_optional() {
        let value = KeywordValue::Optional {
            value: KeywordValue::Atom { atom: "foo".into() }.into(),
        };

        assert_eq!(value.collect_variations(), vec!["", "foo"]);

        let value = KeywordValue::Sequence {
            values: vec![
                KeywordValue::Atom { atom: "foo".into() },
                KeywordValue::Optional {
                    value: KeywordValue::Atom { atom: "bar".into() }.into(),
                },
            ],
        };

        assert_eq!(value.collect_variations(), vec!["foo", "foobar"]);
    }

    #[test]
    fn test_choice() {
        let value = KeywordValue::Choice {
            values: vec![
                KeywordValue::Atom { atom: "foo".into() },
                KeywordValue::Atom { atom: "bar".into() },
            ],
        };

        assert_eq!(value.collect_variations(), vec!["foo", "bar"]);
    }

    #[test]
    fn test_sequence() {
        let value = KeywordValue::Sequence {
            values: vec![
                KeywordValue::Atom { atom: "foo".into() },
                KeywordValue::Atom { atom: "bar".into() },
            ],
        };

        assert_eq!(value.collect_variations(), vec!["foobar"]);
    }

    #[test]
    fn test_all() {
        let value = KeywordValue::Sequence {
            values: vec![
                KeywordValue::Atom { atom: "foo".into() },
                KeywordValue::Optional {
                    value: KeywordValue::Sequence {
                        values: vec![
                            KeywordValue::Atom { atom: "_".into() },
                            KeywordValue::Choice {
                                values: vec![
                                    KeywordValue::Atom { atom: "1".into() },
                                    KeywordValue::Atom { atom: "2".into() },
                                    KeywordValue::Atom { atom: "3".into() },
                                    KeywordValue::Atom { atom: "4".into() },
                                    KeywordValue::Atom { atom: "5".into() },
                                ],
                            },
                        ],
                    }
                    .into(),
                },
            ],
        };

        assert_eq!(
            value.collect_variations(),
            vec!["foo", "foo_1", "foo_2", "foo_3", "foo_4", "foo_5",]
        );
    }

    #[test]
    fn test_product() {
        let value = KeywordValue::Sequence {
            values: vec![
                KeywordValue::Choice {
                    values: vec![
                        KeywordValue::Atom { atom: "1".into() },
                        KeywordValue::Atom { atom: "2".into() },
                        KeywordValue::Atom { atom: "3".into() },
                    ],
                },
                KeywordValue::Atom { atom: "x".into() },
                KeywordValue::Choice {
                    values: vec![
                        KeywordValue::Atom { atom: "1".into() },
                        KeywordValue::Atom { atom: "2".into() },
                        KeywordValue::Atom { atom: "3".into() },
                    ],
                },
                KeywordValue::Atom { atom: "x".into() },
                KeywordValue::Choice {
                    values: vec![
                        KeywordValue::Atom { atom: "1".into() },
                        KeywordValue::Atom { atom: "2".into() },
                        KeywordValue::Atom { atom: "3".into() },
                    ],
                },
            ],
        };

        assert_eq!(
            value.collect_variations(),
            vec![
                "1x1x1", "1x1x2", "1x1x3", "1x2x1", "1x2x2", "1x2x3", "1x3x1", "1x3x2", "1x3x3",
                "2x1x1", "2x1x2", "2x1x3", "2x2x1", "2x2x2", "2x2x3", "2x3x1", "2x3x2", "2x3x3",
                "3x1x1", "3x1x2", "3x1x3", "3x2x1", "3x2x2", "3x2x3", "3x3x1", "3x3x2", "3x3x3",
            ]
        );
    }
}
