use std::ops::Range;
use std::rc::Rc;

use napi::bindgen_prelude::{Env, FromNapiValue, Reference, ToNapiValue};
use napi::{JsFunction, JsObject, JsUnknown, NapiValue};
use napi_derive::napi;

use crate::napi_interface::cursor::Cursor;
use crate::napi_interface::text_index::TextIndex;
use crate::napi_interface::{
    RuleKind, RustNode, RustRuleNode, RustTextIndex, RustTokenNode, TokenKind,
};

#[napi(namespace = "cst")]
pub enum NodeType {
    Rule,
    Token,
}

#[napi(namespace = "cst")]
pub struct RuleNode(pub(crate) Rc<RustRuleNode>);

#[napi(namespace = "cst")]
pub struct TokenNode(pub(crate) Rc<RustTokenNode>);

#[napi(namespace = "cst")]
impl RuleNode {
    #[napi(
        getter,
        js_name = "type",
        ts_return_type = "NodeType.Rule",
        catch_unwind
    )]
    pub fn tipe(&self) -> NodeType {
        NodeType::Rule
    }

    #[napi(getter, ts_return_type = "kinds.RuleKind", catch_unwind)]
    pub fn kind(&self) -> RuleKind {
        self.0.kind
    }

    #[napi(
        getter,
        js_name = "textLength",
        ts_return_type = "text_index.TextIndex",
        catch_unwind
    )]
    pub fn text_len(&self) -> TextIndex {
        (&self.0.text_len).into()
    }

    #[napi(ts_return_type = "Array<cst.Node>", catch_unwind)]
    pub fn children(&self, env: Env) -> Vec<JsObject> {
        self.0
            .children
            .iter()
            .map(|child| child.to_js(&env))
            .collect()
    }

    #[napi(ts_return_type = "cursor.Cursor", catch_unwind)]
    pub fn create_cursor(
        &self,
        #[napi(ts_arg_type = "text_index.TextIndex")] text_offset: TextIndex,
    ) -> Cursor {
        RustNode::Rule(self.0.clone())
            .cursor_with_offset((&text_offset).into())
            .into()
    }

    #[napi(catch_unwind, js_name = "toJSON")]
    /// Serialize the token node to JSON.
    ///
    /// This method is intended for debugging purposes and may not be stable.
    pub fn to_json(&self) -> String {
        serde_json::to_string(&self.0).unwrap()
    }

    #[napi(catch_unwind)]
    pub fn unparse(&self) -> String {
        self.0.clone().unparse()
    }

    // Expose the children as a hidden (non-enumerable, don't generate type definition)
    // property that's eagerly evaluated (getter) in the debugger context.
    #[napi(
        enumerable = false,
        configurable = false,
        writable = false,
        getter,
        js_name = "__children", // Needed; otherwise, the property name would shadow `children`.
        skip_typescript
    )]
    pub fn __children(&self, env: Env) -> Vec<JsObject> {
        Self::children(self, env)
    }

    // Similarly, expose the eagerly evaluated unparsed text in the debugger context.
    #[napi(
        enumerable = false,
        configurable = false,
        writable = false,
        getter,
        js_name = "__text",
        skip_typescript
    )]
    pub fn __text(&self) -> String {
        self.unparse()
    }
}

#[napi(namespace = "cst")]
impl TokenNode {
    #[napi(
        getter,
        js_name = "type",
        ts_return_type = "NodeType.Token",
        catch_unwind
    )]
    pub fn tipe(&self) -> NodeType {
        NodeType::Token
    }

    #[napi(getter, ts_return_type = "kinds.TokenKind", catch_unwind)]
    pub fn kind(&self) -> TokenKind {
        self.0.kind
    }

    #[napi(
        getter,
        js_name = "textLength",
        ts_return_type = "text_index.TextIndex",
        catch_unwind
    )]
    pub fn text_len(&self) -> TextIndex {
        let text_len: RustTextIndex = (&self.0.text).into();
        (&text_len).into()
    }

    #[napi(getter, catch_unwind)]
    pub fn text(&self) -> String {
        self.0.text.clone()
    }

    #[napi(catch_unwind, js_name = "toJSON")]
    /// Serialize the token node to JSON.
    ///
    /// This method is intended for debugging purposes and may not be stable.
    pub fn to_json(&self) -> String {
        serde_json::to_string(&self.0).unwrap()
    }

    #[napi(ts_return_type = "cursor.Cursor", catch_unwind)]
    pub fn create_cursor(
        &self,
        #[napi(ts_arg_type = "text_index.TextIndex")] text_offset: TextIndex,
    ) -> Cursor {
        RustNode::Token(self.0.clone())
            .cursor_with_offset((&text_offset).into())
            .into()
    }
}

pub trait ToJS {
    fn to_js(&self, env: &Env) -> JsObject;
}

impl ToJS for Rc<RustRuleNode> {
    fn to_js(&self, env: &Env) -> JsObject {
        let obj =
            unsafe { <RuleNode as ToNapiValue>::to_napi_value(env.raw(), RuleNode(self.clone())) };
        let mut obj = unsafe { JsObject::from_raw_unchecked(env.raw(), obj.unwrap()) };

        // TODO: Replace with `env.symbol_for` available under `napi9` feature flag;
        // This feature is available from Node v16.15.0, which is shipped starting from VSCode 1.78 (April 2023).
        let global = env.get_global().unwrap();
        let symbol = global.get_named_property::<JsFunction>("Symbol").unwrap();
        let symbol = symbol.coerce_to_object().unwrap();
        let symbol_for_fn = symbol.get_named_property::<JsFunction>("for").unwrap();
        let symbol_desc = env.create_string("nodejs.util.inspect.custom").unwrap();
        let inspect_symbol = symbol_for_fn.call(Some(&symbol), &[symbol_desc]).unwrap();

        obj.set_property(
            inspect_symbol,
            env.create_function_from_closure("inspect", move |ctx| {
                // https://github.com/nodejs/node/pull/41019
                // TODO: Support more:
                // args: (depth, inspectOptions, inspect)
                let this: JsUnknown = ctx.this()?;
                let rule = Reference::<RuleNode>::from_unknown(this)?;

                let contents = rule.unparse();
                let preview = render_debug_preview(&contents, 0..rule.0.text_len.utf8);

                ctx.env
                    .create_string(&format!("{kind} (Rule): {preview}", kind = rule.0.kind))
            })
            .unwrap(),
        )
        .unwrap();

        obj
    }
}

impl ToJS for Rc<RustTokenNode> {
    fn to_js(&self, env: &Env) -> JsObject {
        let obj = unsafe {
            <TokenNode as ToNapiValue>::to_napi_value(env.raw(), TokenNode(self.clone()))
        };
        let mut obj = unsafe { JsObject::from_raw_unchecked(env.raw(), obj.unwrap()) };

        // TODO: Replace with `env.symbol_for` available under `napi9` feature flag;
        // This feature is available from Node v16.15.0, which is shipped starting from VSCode 1.78 (April 2023).
        let global = env.get_global().unwrap();
        let symbol = global.get_named_property::<JsFunction>("Symbol").unwrap();
        let symbol = symbol.coerce_to_object().unwrap();
        let symbol_for_fn = symbol.get_named_property::<JsFunction>("for").unwrap();
        let symbol_desc = env.create_string("nodejs.util.inspect.custom").unwrap();
        let inspect_symbol = symbol_for_fn.call(Some(&symbol), &[symbol_desc]).unwrap();

        obj.set_property(
            inspect_symbol,
            env.create_function_from_closure("inspect", move |ctx| {
                // https://github.com/nodejs/node/pull/41019
                // TODO: Support more:
                // args: (depth, inspectOptions, inspect)
                let this: JsUnknown = ctx.this()?;
                let token = Reference::<TokenNode>::from_unknown(this)?;

                let contents = token.text();
                let preview = render_debug_preview(&contents, 0..contents.len());

                ctx.env
                    .create_string(&format!("{kind} (Token): {preview}", kind = token.0.kind))
            })
            .unwrap(),
        )
        .unwrap();

        obj
    }
}

impl ToJS for RustNode {
    fn to_js(&self, env: &Env) -> JsObject {
        match self {
            RustNode::Rule(rust_rule_node) => rust_rule_node.to_js(env),
            RustNode::Token(rust_token_node) => rust_token_node.to_js(env),
        }
    }
}

fn render_debug_preview(source: &str, char_range: Range<usize>) -> String {
    let length = char_range.len();

    // Trim long values:
    let max_length = 50;
    let mut contents: String = source
        .chars()
        .skip(char_range.start)
        .take(std::cmp::min(length, max_length))
        .collect();

    // Add terminator if trimmed:
    if length > max_length {
        contents.push_str("...");
    }

    // Escape line breaks:
    let contents = contents
        .replace('\t', "\\t")
        .replace('\r', "\\r")
        .replace('\n', "\\n");

    // Surround by quotes for use in yaml:
    if contents.contains('"') {
        let contents = contents.replace('\'', "''");
        format!("'{contents}'")
    } else {
        let contents = contents.replace('"', "\\\"");
        format!("\"{contents}\"")
    }
}
