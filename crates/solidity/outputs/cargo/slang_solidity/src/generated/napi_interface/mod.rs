// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

pub mod ast_selectors;
pub mod cst;
pub mod cursor;
pub mod parse_error;
pub mod parse_output;
pub mod query;
pub mod text_index;

#[napi_derive::module_exports]
fn init(_exports: napi::JsObject, env: napi::Env) -> napi::Result<()> {
    struct DynThreadLocalEnv(napi::Env, std::thread::ThreadId);
    impl DynThreadLocalEnv {
        fn wrap(env: napi::Env) -> Self {
            Self(env, std::thread::current().id())
        }
        fn expose(&self) -> Option<&napi::Env> {
            (std::thread::current().id() == self.1).then_some(&self.0)
        }
    }
    unsafe impl Send for DynThreadLocalEnv {}
    unsafe impl Sync for DynThreadLocalEnv {}

    let env = DynThreadLocalEnv::wrap(env);

    std::panic::set_hook(Box::new(move |info| {
        let location = info.location().unwrap();
        let location = location.to_string();
        let message = info.to_string();

        // FIXME: This will be called even if we unwind and there's no way to see if the panic is in fact fatal.
        if let Some(env) = env.expose() {
            env.fatal_error(&location, &message);
        }
    }));

    Ok(())
}

type RustCursor = crate::cursor::Cursor;
type RustLabeledNode = crate::cst::LabeledNode;
type RustNode = crate::cst::Node;
type RustParseError = crate::parse_error::ParseError;
type RustParseOutput = crate::parse_output::ParseOutput;
type RustQuery = crate::query::Query;
type RustQueryResult = crate::query::QueryResult;
type RustQueryResultIterator = crate::query::QueryResultIterator;
type RustRuleNode = crate::cst::RuleNode;
type RustTextIndex = crate::text_index::TextIndex;
type RustTextRange = crate::text_index::TextRange;
type RustTokenNode = crate::cst::TokenNode;

type RuleKind = crate::kinds::RuleKind;
type TokenKind = crate::kinds::TokenKind;
type NodeLabel = crate::kinds::NodeLabel;
