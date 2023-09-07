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
    ArgumentsDeclaration,
    ArrayExpression,
    ArrayValuesList,
    AsciiStringLiteralsList,
    AssemblyFlagsList,
    AssemblyStatement,
    Block,
    BreakStatement,
    CatchClause,
    CatchClauseError,
    CatchClausesList,
    ConstantDefinition,
    ConstructorAttributesList,
    ConstructorDefinition,
    ContinueStatement,
    ContractDefinition,
    ContractMembersList,
    DeconstructionImport,
    DeconstructionImportSymbol,
    DeconstructionImportSymbolsList,
    DeleteStatement,
    DoWhileStatement,
    EmitStatement,
    EndOfFileTrivia,
    EnumDefinition,
    ErrorDefinition,
    ErrorParameter,
    ErrorParametersList,
    EventDefinition,
    EventParameter,
    EventParametersList,
    ExperimentalPragma,
    Expression,
    ExpressionStatement,
    FallbackFunctionAttributesList,
    FallbackFunctionDefinition,
    ForStatement,
    FunctionAttributesList,
    FunctionCallOptions,
    FunctionDefinition,
    FunctionType,
    FunctionTypeAttributesList,
    HexStringLiteralsList,
    IdentifierPath,
    IdentifierPathsList,
    IdentifiersList,
    IfStatement,
    ImportDirective,
    InheritanceSpecifier,
    InheritanceType,
    InheritanceTypesList,
    InterfaceDefinition,
    InterfaceMembersList,
    LeadingTrivia,
    LibraryDefinition,
    LibraryMembersList,
    MappingKeyType,
    MappingType,
    MappingValueType,
    ModifierAttributesList,
    ModifierDefinition,
    ModifierInvocation,
    NamedArgument,
    NamedArgumentsDeclaration,
    NamedArgumentsList,
    NamedImport,
    NewExpression,
    NumericExpression,
    OverrideSpecifier,
    Parameter,
    ParametersDeclaration,
    ParametersList,
    PathImport,
    PositionalArgumentsList,
    PragmaDirective,
    ReceiveFunctionAttributesList,
    ReceiveFunctionDefinition,
    ReturnStatement,
    ReturnsDeclaration,
    RevertStatement,
    SourceUnit,
    SourceUnitMembersList,
    StateVariableAttributesList,
    StateVariableDefinition,
    Statement,
    StatementsList,
    StructDefinition,
    StructMember,
    StructMembersList,
    ThrowStatement,
    TrailingTrivia,
    TryStatement,
    TupleDeconstructionStatement,
    TupleExpression,
    TupleMember,
    TupleMembersList,
    TupleValuesList,
    TypeExpression,
    TypeName,
    UncheckedBlock,
    UnicodeStringLiteralsList,
    UnnamedFunctionAttributesList,
    UnnamedFunctionDefinition,
    UserDefinedValueTypeDefinition,
    UsingDirective,
    UsingDirectiveDeconstruction,
    UsingDirectivePath,
    UsingDirectiveSymbol,
    UsingDirectiveSymbolsList,
    VariableDeclaration,
    VariableDeclarationStatement,
    VersionPragma,
    VersionPragmaExpression,
    VersionPragmaExpressionsList,
    VersionPragmaSpecifier,
    WhileStatement,
    YulAssignmentStatement,
    YulBlock,
    YulBreakStatement,
    YulContinueStatement,
    YulDeclarationStatement,
    YulExpression,
    YulExpressionsList,
    YulForStatement,
    YulFunctionDefinition,
    YulIdentifierPath,
    YulIdentifierPathsList,
    YulIdentifiersList,
    YulIfStatement,
    YulLeaveStatement,
    YulParametersDeclaration,
    YulReturnsDeclaration,
    YulStatement,
    YulStatementsList,
    YulSwitchCase,
    YulSwitchCasesList,
    YulSwitchStatement,
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
    ArgumentsDeclaration,
    ArrayExpression,
    ArrayTypeName,
    ArrayValuesList,
    AsciiStringLiteralsList,
    AssemblyFlagsList,
    AssemblyStatement,
    BinaryExpression,
    Block,
    BreakStatement,
    CatchClause,
    CatchClauseError,
    CatchClausesList,
    ConditionalExpression,
    ConstantDefinition,
    ConstructorAttributesList,
    ConstructorDefinition,
    ContinueStatement,
    ContractDefinition,
    ContractMembersList,
    DeconstructionImport,
    DeconstructionImportSymbol,
    DeconstructionImportSymbolsList,
    DeleteStatement,
    DoWhileStatement,
    EmitStatement,
    EndOfFileTrivia,
    EnumDefinition,
    ErrorDefinition,
    ErrorParameter,
    ErrorParametersList,
    EventDefinition,
    EventParameter,
    EventParametersList,
    ExperimentalPragma,
    Expression,
    ExpressionStatement,
    FallbackFunctionAttributesList,
    FallbackFunctionDefinition,
    ForStatement,
    FunctionAttributesList,
    FunctionCallExpression,
    FunctionCallOptions,
    FunctionDefinition,
    FunctionType,
    FunctionTypeAttributesList,
    HexStringLiteralsList,
    IdentifierPath,
    IdentifierPathsList,
    IdentifiersList,
    IfStatement,
    ImportDirective,
    IndexAccessExpression,
    InheritanceSpecifier,
    InheritanceType,
    InheritanceTypesList,
    InterfaceDefinition,
    InterfaceMembersList,
    LeadingTrivia,
    LibraryDefinition,
    LibraryMembersList,
    MappingKeyType,
    MappingType,
    MappingValueType,
    MemberAccessExpression,
    ModifierAttributesList,
    ModifierDefinition,
    ModifierInvocation,
    NamedArgument,
    NamedArgumentsDeclaration,
    NamedArgumentsList,
    NamedImport,
    NewExpression,
    NumericExpression,
    OverrideSpecifier,
    Parameter,
    ParametersDeclaration,
    ParametersList,
    PathImport,
    PositionalArgumentsList,
    PragmaDirective,
    ReceiveFunctionAttributesList,
    ReceiveFunctionDefinition,
    ReturnStatement,
    ReturnsDeclaration,
    RevertStatement,
    SourceUnit,
    SourceUnitMembersList,
    StateVariableAttributesList,
    StateVariableDefinition,
    Statement,
    StatementsList,
    StructDefinition,
    StructMember,
    StructMembersList,
    ThrowStatement,
    TrailingTrivia,
    TryStatement,
    TupleDeconstructionStatement,
    TupleExpression,
    TupleMember,
    TupleMembersList,
    TupleValuesList,
    TypeExpression,
    TypeName,
    UnaryPostfixExpression,
    UnaryPrefixExpression,
    UncheckedBlock,
    UnicodeStringLiteralsList,
    UnnamedFunctionAttributesList,
    UnnamedFunctionDefinition,
    UserDefinedValueTypeDefinition,
    UsingDirective,
    UsingDirectiveDeconstruction,
    UsingDirectivePath,
    UsingDirectiveSymbol,
    UsingDirectiveSymbolsList,
    VariableDeclaration,
    VariableDeclarationStatement,
    VersionPragma,
    VersionPragmaBinaryExpression,
    VersionPragmaExpression,
    VersionPragmaExpressionsList,
    VersionPragmaSpecifier,
    VersionPragmaUnaryExpression,
    WhileStatement,
    YulAssignmentStatement,
    YulBlock,
    YulBreakStatement,
    YulContinueStatement,
    YulDeclarationStatement,
    YulExpression,
    YulExpressionsList,
    YulForStatement,
    YulFunctionCallExpression,
    YulFunctionDefinition,
    YulIdentifierPath,
    YulIdentifierPathsList,
    YulIdentifiersList,
    YulIfStatement,
    YulLeaveStatement,
    YulParametersDeclaration,
    YulReturnsDeclaration,
    YulStatement,
    YulStatementsList,
    YulSwitchCase,
    YulSwitchCasesList,
    YulSwitchStatement,
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
    ABICoderKeyword,
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
    CalldataKeyword,
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
    CopyofKeyword,
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
    FixedBytesType,
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
    InterfaceKeyword,
    InternalKeyword,
    IsKeyword,
    LeaveKeyword,
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
    SignedFixedType,
    SignedIntegerType,
    SingleLineComment,
    SizeofKeyword,
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
    TypeKeyword,
    TypedefKeyword,
    TypeofKeyword,
    UncheckedKeyword,
    UnicodeStringLiteral,
    UnsignedFixedType,
    UnsignedIntegerType,
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
    YulDecimalLiteral,
    YulHexLiteral,
    YulIdentifier,
}

impl TokenKind {
    pub fn is_whitespace(&self) -> bool {
        matches!(self, TokenKind::Whitespace | TokenKind::EndOfLine)
    }
}

#[derive(strum_macros::FromRepr)]
/// The lexical context of the scanner.
#[cfg_attr(feature = "slang_napi_interfaces", /* derives `Clone` and `Copy` */ napi(string_enum, namespace = "language"))]
#[cfg_attr(not(feature = "slang_napi_interfaces"), derive(Clone, Copy))]
#[repr(u8)] // This is used as a const argument, which only supports primitive types
pub enum LexicalContext {
    Default,
    VersionPragma,
    YulBlock,
}
