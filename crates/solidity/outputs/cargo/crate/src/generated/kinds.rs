// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

#[cfg(feature = "slang_napi_interfaces")]
use {napi::bindgen_prelude::*, napi_derive::napi};

#[derive(
    Debug,
    Eq,
    Ord,
    PartialEq,
    PartialOrd,
    serde::Serialize,
    strum_macros::AsRefStr,
    strum_macros::Display,
    strum_macros::EnumString,
)]
#[cfg_attr(feature = "slang_napi_interfaces", /* derives `Clone` and `Copy` */ napi(string_enum, namespace = "kinds"))]
#[cfg_attr(not(feature = "slang_napi_interfaces"), derive(Clone, Copy))]
pub enum ProductionKind {
    ABICoderPragma,
    AddressType,
    ArrayExpression,
    ArrayValues,
    AsciiStringLiterals,
    AssemblyFlags,
    AssemblyFlagsDeclaration,
    AssemblyStatement,
    Block,
    BreakStatement,
    CatchClause,
    CatchClauseError,
    CatchClauses,
    ConstantDefinition,
    ConstructorAttributes,
    ConstructorDefinition,
    ContinueStatement,
    ContractDefinition,
    ContractMembers,
    DecimalNumberExpression,
    DeleteStatement,
    DoWhileStatement,
    ElseBranch,
    EmitStatement,
    EndOfFileTrivia,
    EnumDefinition,
    EnumMembers,
    ErrorDefinition,
    ErrorParameter,
    ErrorParameters,
    ErrorParametersDeclaration,
    EventDefinition,
    EventParameter,
    EventParameters,
    EventParametersDeclaration,
    ExperimentalPragma,
    Expression,
    ExpressionStatement,
    FallbackFunctionAttributes,
    FallbackFunctionDefinition,
    ForStatement,
    FunctionAttributes,
    FunctionDefinition,
    FunctionType,
    FunctionTypeAttributes,
    HexNumberExpression,
    HexStringLiterals,
    IdentifierPath,
    IfStatement,
    ImportAlias,
    ImportDeconstructionField,
    ImportDeconstructionFields,
    ImportDirective,
    ImportSymbolDeconstruction,
    IndexAccessEnd,
    InheritanceSpecifier,
    InheritanceType,
    InheritanceTypes,
    InterfaceDefinition,
    InterfaceMembers,
    LeadingTrivia,
    LibraryDefinition,
    LibraryMembers,
    MappingKey,
    MappingType,
    MappingValue,
    ModifierAttributes,
    ModifierDefinition,
    ModifierInvocation,
    NamedArgument,
    NamedArgumentGroup,
    NamedArgumentGroups,
    NamedArguments,
    NamedArgumentsDeclaration,
    NamedImportSymbol,
    NewExpression,
    OverridePaths,
    OverridePathsDeclaration,
    OverrideSpecifier,
    Parameter,
    Parameters,
    ParametersDeclaration,
    PathImportSymbol,
    PositionalArguments,
    PositionalArgumentsDeclaration,
    PragmaDirective,
    ReceiveFunctionAttributes,
    ReceiveFunctionDefinition,
    ReturnStatement,
    ReturnsDeclaration,
    RevertStatement,
    SourceUnit,
    SourceUnitMembers,
    StateVariableAttributes,
    StateVariableDefinition,
    StateVariableDefinitionValue,
    Statements,
    StructDefinition,
    StructMember,
    StructMembers,
    ThrowStatement,
    TrailingTrivia,
    TryStatement,
    TupleDeconstructionStatement,
    TupleExpression,
    TupleMemberDeconstruction,
    TupleMembersDeconstruction,
    TupleValue,
    TupleValues,
    TypeExpression,
    TypeName,
    TypedTupleMember,
    UncheckedBlock,
    UnicodeStringLiterals,
    UnnamedFunctionAttributes,
    UnnamedFunctionDefinition,
    UntypedTupleMember,
    UserDefinedValueTypeDefinition,
    UsingAlias,
    UsingDeconstructionField,
    UsingDeconstructionFields,
    UsingDirective,
    UsingSymbolDeconstruction,
    VariableDeclarationStatement,
    VariableDeclarationValue,
    VersionPragma,
    VersionPragmaExpression,
    VersionPragmaExpressions,
    VersionPragmaSpecifier,
    WhileStatement,
    YulArguments,
    YulAssignmentStatement,
    YulBlock,
    YulBreakStatement,
    YulContinueStatement,
    YulDefaultCase,
    YulExpression,
    YulForStatement,
    YulFunctionDefinition,
    YulIdentifierPath,
    YulIdentifierPaths,
    YulIfStatement,
    YulLeaveStatement,
    YulParameters,
    YulParametersDeclaration,
    YulReturnVariables,
    YulReturnsDeclaration,
    YulStatements,
    YulSwitchCases,
    YulSwitchStatement,
    YulValueCase,
    YulVariableDeclarationStatement,
    YulVariableDeclarationValue,
}

#[derive(
    Debug,
    Eq,
    Ord,
    PartialEq,
    PartialOrd,
    serde::Serialize,
    strum_macros::AsRefStr,
    strum_macros::Display,
    strum_macros::EnumString,
)]
#[cfg_attr(feature = "slang_napi_interfaces", /* derives `Clone` and `Copy` */ napi(string_enum, namespace = "kinds"))]
#[cfg_attr(not(feature = "slang_napi_interfaces"), derive(Clone, Copy))]
pub enum RuleKind {
    ABICoderPragma,
    AddressType,
    ArrayExpression,
    ArrayTypeName,
    ArrayValues,
    AsciiStringLiterals,
    AssemblyFlags,
    AssemblyFlagsDeclaration,
    AssemblyStatement,
    BinaryExpression,
    Block,
    BreakStatement,
    CatchClause,
    CatchClauseError,
    CatchClauses,
    ConditionalExpression,
    ConstantDefinition,
    ConstructorAttributes,
    ConstructorDefinition,
    ContinueStatement,
    ContractDefinition,
    ContractMembers,
    DecimalNumberExpression,
    DeleteStatement,
    DoWhileStatement,
    ElseBranch,
    EmitStatement,
    EndOfFileTrivia,
    EnumDefinition,
    EnumMembers,
    ErrorDefinition,
    ErrorParameter,
    ErrorParameters,
    ErrorParametersDeclaration,
    EventDefinition,
    EventParameter,
    EventParameters,
    EventParametersDeclaration,
    ExperimentalPragma,
    Expression,
    ExpressionStatement,
    FallbackFunctionAttributes,
    FallbackFunctionDefinition,
    ForStatement,
    FunctionAttributes,
    FunctionCallExpression,
    FunctionDefinition,
    FunctionType,
    FunctionTypeAttributes,
    HexNumberExpression,
    HexStringLiterals,
    IdentifierPath,
    IfStatement,
    ImportAlias,
    ImportDeconstructionField,
    ImportDeconstructionFields,
    ImportDirective,
    ImportSymbolDeconstruction,
    IndexAccessEnd,
    IndexAccessExpression,
    InheritanceSpecifier,
    InheritanceType,
    InheritanceTypes,
    InterfaceDefinition,
    InterfaceMembers,
    LeadingTrivia,
    LibraryDefinition,
    LibraryMembers,
    MappingKey,
    MappingType,
    MappingValue,
    MemberAccessExpression,
    ModifierAttributes,
    ModifierDefinition,
    ModifierInvocation,
    NamedArgument,
    NamedArgumentGroup,
    NamedArgumentGroups,
    NamedArguments,
    NamedArgumentsDeclaration,
    NamedImportSymbol,
    NewExpression,
    OverridePaths,
    OverridePathsDeclaration,
    OverrideSpecifier,
    Parameter,
    Parameters,
    ParametersDeclaration,
    PathImportSymbol,
    PositionalArguments,
    PositionalArgumentsDeclaration,
    PragmaDirective,
    ReceiveFunctionAttributes,
    ReceiveFunctionDefinition,
    ReturnStatement,
    ReturnsDeclaration,
    RevertStatement,
    SourceUnit,
    SourceUnitMembers,
    StateVariableAttributes,
    StateVariableDefinition,
    StateVariableDefinitionValue,
    Statements,
    StructDefinition,
    StructMember,
    StructMembers,
    ThrowStatement,
    TrailingTrivia,
    TryStatement,
    TupleDeconstructionStatement,
    TupleExpression,
    TupleMemberDeconstruction,
    TupleMembersDeconstruction,
    TupleValue,
    TupleValues,
    TypeExpression,
    TypeName,
    TypedTupleMember,
    UnaryPostfixExpression,
    UnaryPrefixExpression,
    UncheckedBlock,
    UnicodeStringLiterals,
    UnnamedFunctionAttributes,
    UnnamedFunctionDefinition,
    UntypedTupleMember,
    UserDefinedValueTypeDefinition,
    UsingAlias,
    UsingDeconstructionField,
    UsingDeconstructionFields,
    UsingDirective,
    UsingSymbolDeconstruction,
    VariableDeclarationStatement,
    VariableDeclarationValue,
    VersionPragma,
    VersionPragmaBinaryExpression,
    VersionPragmaExpression,
    VersionPragmaExpressions,
    VersionPragmaSpecifier,
    VersionPragmaUnaryExpression,
    WhileStatement,
    YulArguments,
    YulAssignmentStatement,
    YulBlock,
    YulBreakStatement,
    YulContinueStatement,
    YulDefaultCase,
    YulExpression,
    YulForStatement,
    YulFunctionCallExpression,
    YulFunctionDefinition,
    YulIdentifierPath,
    YulIdentifierPaths,
    YulIfStatement,
    YulLeaveStatement,
    YulParameters,
    YulParametersDeclaration,
    YulReturnVariables,
    YulReturnsDeclaration,
    YulStatements,
    YulSwitchCases,
    YulSwitchStatement,
    YulValueCase,
    YulVariableDeclarationStatement,
    YulVariableDeclarationValue,
}

impl RuleKind {
    pub fn is_trivia(&self) -> bool {
        match self {
            Self::EndOfFileTrivia => true,
            Self::LeadingTrivia => true,
            Self::TrailingTrivia => true,
            _ => false,
        }
    }
}

#[derive(
    Debug,
    Eq,
    Ord,
    PartialEq,
    PartialOrd,
    serde::Serialize,
    strum_macros::AsRefStr,
    strum_macros::Display,
    strum_macros::EnumString,
)]
#[cfg_attr(feature = "slang_napi_interfaces", /* derives `Clone` and `Copy` */ napi(string_enum, namespace = "kinds"))]
#[cfg_attr(not(feature = "slang_napi_interfaces"), derive(Clone, Copy))]
pub enum TokenKind {
    SKIPPED,
    AbicoderKeyword,
    AbstractKeyword,
    AddressKeyword,
    AfterKeyword,
    AliasKeyword,
    Ampersand,
    AmpersandAmpersand,
    AmpersandEqual,
    AnonymousKeyword,
    ApplyKeyword,
    AsKeyword,
    AsciiStringLiteral,
    AssemblyKeyword,
    Asterisk,
    AsteriskAsterisk,
    AsteriskEqual,
    AutoKeyword,
    Bang,
    BangEqual,
    Bar,
    BarBar,
    BarEqual,
    BoolKeyword,
    BreakKeyword,
    ByteKeyword,
    BytesKeyword,
    CallDataKeyword,
    Caret,
    CaretEqual,
    CaseKeyword,
    CatchKeyword,
    CloseBrace,
    CloseBracket,
    CloseParen,
    Colon,
    ColonEqual,
    Comma,
    ConstantKeyword,
    ConstructorKeyword,
    ContinueKeyword,
    ContractKeyword,
    CopyOfKeyword,
    DaysKeyword,
    DecimalLiteral,
    DefaultKeyword,
    DefineKeyword,
    DeleteKeyword,
    DoKeyword,
    ElseKeyword,
    EmitKeyword,
    EndOfLine,
    EnumKeyword,
    Equal,
    EqualEqual,
    EqualGreaterThan,
    ErrorKeyword,
    EtherKeyword,
    EventKeyword,
    ExperimentalKeyword,
    ExternalKeyword,
    FallbackKeyword,
    FalseKeyword,
    FinalKeyword,
    FinneyKeyword,
    FixedKeyword,
    ForKeyword,
    FromKeyword,
    FunctionKeyword,
    GlobalKeyword,
    GreaterThan,
    GreaterThanEqual,
    GreaterThanGreaterThan,
    GreaterThanGreaterThanEqual,
    GreaterThanGreaterThanGreaterThan,
    GreaterThanGreaterThanGreaterThanEqual,
    GweiKeyword,
    HexKeyword,
    HexLiteral,
    HexStringLiteral,
    HoursKeyword,
    Identifier,
    IfKeyword,
    ImmutableKeyword,
    ImplementsKeyword,
    ImportKeyword,
    InKeyword,
    IndexedKeyword,
    InlineKeyword,
    IntKeyword,
    InterfaceKeyword,
    InternalKeyword,
    IsKeyword,
    LessThan,
    LessThanEqual,
    LessThanLessThan,
    LessThanLessThanEqual,
    LetKeyword,
    LibraryKeyword,
    MacroKeyword,
    MappingKeyword,
    MatchKeyword,
    MemoryKeyword,
    Minus,
    MinusEqual,
    MinusGreaterThan,
    MinusMinus,
    MinutesKeyword,
    ModifierKeyword,
    MultilineComment,
    MutableKeyword,
    NewKeyword,
    NullKeyword,
    OfKeyword,
    OpenBrace,
    OpenBracket,
    OpenParen,
    OverrideKeyword,
    PartialKeyword,
    PayableKeyword,
    Percent,
    PercentEqual,
    Period,
    Plus,
    PlusEqual,
    PlusPlus,
    PragmaKeyword,
    PrivateKeyword,
    PromiseKeyword,
    PublicKeyword,
    PureKeyword,
    QuestionMark,
    ReceiveKeyword,
    ReferenceKeyword,
    RelocatableKeyword,
    ReturnKeyword,
    ReturnsKeyword,
    RevertKeyword,
    SealedKeyword,
    SecondsKeyword,
    Semicolon,
    SingleLineComment,
    SizeOfKeyword,
    Slash,
    SlashEqual,
    SolidityKeyword,
    StaticKeyword,
    StorageKeyword,
    StringKeyword,
    StructKeyword,
    SupportsKeyword,
    SwitchKeyword,
    SzaboKeyword,
    ThrowKeyword,
    Tilde,
    TrueKeyword,
    TryKeyword,
    TypeDefKeyword,
    TypeKeyword,
    TypeOfKeyword,
    UfixedKeyword,
    UintKeyword,
    UncheckedKeyword,
    UnicodeStringLiteral,
    UsingKeyword,
    VarKeyword,
    VersionPragmaValue,
    ViewKeyword,
    VirtualKeyword,
    WeeksKeyword,
    WeiKeyword,
    WhileKeyword,
    Whitespace,
    YearsKeyword,
    YulAbstractKeyword,
    YulAddressKeyword,
    YulAfterKeyword,
    YulAliasKeyword,
    YulAnonymousKeyword,
    YulApplyKeyword,
    YulAsKeyword,
    YulAssemblyKeyword,
    YulAutoKeyword,
    YulBoolKeyword,
    YulBreakKeyword,
    YulByteKeyword,
    YulBytesKeyword,
    YulCallDataKeyword,
    YulCaseKeyword,
    YulCatchKeyword,
    YulConstantKeyword,
    YulConstructorKeyword,
    YulContinueKeyword,
    YulContractKeyword,
    YulCopyOfKeyword,
    YulDaysKeyword,
    YulDecimalLiteral,
    YulDefaultKeyword,
    YulDefineKeyword,
    YulDeleteKeyword,
    YulDoKeyword,
    YulElseKeyword,
    YulEmitKeyword,
    YulEnumKeyword,
    YulEtherKeyword,
    YulEventKeyword,
    YulExternalKeyword,
    YulFallbackKeyword,
    YulFalseKeyword,
    YulFinalKeyword,
    YulFinneyKeyword,
    YulFixedKeyword,
    YulForKeyword,
    YulFunctionKeyword,
    YulGweiKeyword,
    YulHexKeyword,
    YulHexLiteral,
    YulHoursKeyword,
    YulIdentifier,
    YulIfKeyword,
    YulImmutableKeyword,
    YulImplementsKeyword,
    YulImportKeyword,
    YulInKeyword,
    YulIndexedKeyword,
    YulInlineKeyword,
    YulIntKeyword,
    YulInterfaceKeyword,
    YulInternalKeyword,
    YulIsKeyword,
    YulLeaveKeyword,
    YulLetKeyword,
    YulLibraryKeyword,
    YulMacroKeyword,
    YulMappingKeyword,
    YulMatchKeyword,
    YulMemoryKeyword,
    YulMinutesKeyword,
    YulModifierKeyword,
    YulMutableKeyword,
    YulNewKeyword,
    YulNullKeyword,
    YulOfKeyword,
    YulOverrideKeyword,
    YulPartialKeyword,
    YulPayableKeyword,
    YulPragmaKeyword,
    YulPrivateKeyword,
    YulPromiseKeyword,
    YulPublicKeyword,
    YulPureKeyword,
    YulReceiveKeyword,
    YulReferenceKeyword,
    YulRelocatableKeyword,
    YulReturnKeyword,
    YulReturnsKeyword,
    YulRevertKeyword,
    YulSealedKeyword,
    YulSecondsKeyword,
    YulSizeOfKeyword,
    YulStaticKeyword,
    YulStorageKeyword,
    YulStringKeyword,
    YulStructKeyword,
    YulSupportsKeyword,
    YulSwitchKeyword,
    YulSzaboKeyword,
    YulThrowKeyword,
    YulTrueKeyword,
    YulTryKeyword,
    YulTypeDefKeyword,
    YulTypeKeyword,
    YulTypeOfKeyword,
    YulUfixedKeyword,
    YulUintKeyword,
    YulUncheckedKeyword,
    YulUsingKeyword,
    YulVarKeyword,
    YulViewKeyword,
    YulVirtualKeyword,
    YulWeeksKeyword,
    YulWeiKeyword,
    YulWhileKeyword,
    YulYearsKeyword,
}

#[derive(strum_macros::FromRepr)]
/// The lexical context of the scanner.
#[cfg_attr(feature = "slang_napi_interfaces", /* derives `Clone` and `Copy` */ napi(string_enum, namespace = "language"))]
#[cfg_attr(not(feature = "slang_napi_interfaces"), derive(Clone, Copy))]
pub enum LexicalContext {
    Default,
    Pragma,
    Yul,
}

/// Marker trait for type-level [`LexicalContext`] variants.
pub trait IsLexicalContext {
    /// Returns a run-time [`LexicalContext`] value.
    fn value() -> LexicalContext;
}

#[allow(non_snake_case)]
pub mod LexicalContextType {
    use super::*;
    pub struct Default {}
    impl IsLexicalContext for Default {
        fn value() -> LexicalContext {
            LexicalContext::Default
        }
    }
    pub struct Pragma {}
    impl IsLexicalContext for Pragma {
        fn value() -> LexicalContext {
            LexicalContext::Pragma
        }
    }
    pub struct Yul {}
    impl IsLexicalContext for Yul {
        fn value() -> LexicalContext {
            LexicalContext::Yul
        }
    }
}
