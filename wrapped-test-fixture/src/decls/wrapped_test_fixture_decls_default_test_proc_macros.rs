use serde::{Deserialize, Serialize};
use std::collections::HashMap;
fn default_test_proc_macros() -> Box<[(String, ProcMacro)]> {
    Box::new([
        (
            r#"
#[proc_macro_attribute]
pub fn identity(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}
"#
                .into(),
            ProcMacro {
                name: Symbol::intern("identity"),
                kind: ProcMacroKind::Attr,
                expander: sync::Arc::new(IdentityProcMacroExpander),
                disabled: false,
            },
        ),
        (
            r#"
#[proc_macro_derive(DeriveIdentity)]
pub fn derive_identity(item: TokenStream) -> TokenStream {
    item
}
"#
                .into(),
            ProcMacro {
                name: Symbol::intern("DeriveIdentity"),
                kind: ProcMacroKind::CustomDerive,
                expander: sync::Arc::new(IdentityProcMacroExpander),
                disabled: false,
            },
        ),
        (
            r#"
#[proc_macro_attribute]
pub fn input_replace(attr: TokenStream, _item: TokenStream) -> TokenStream {
    attr
}
"#
                .into(),
            ProcMacro {
                name: Symbol::intern("input_replace"),
                kind: ProcMacroKind::Attr,
                expander: sync::Arc::new(AttributeInputReplaceProcMacroExpander),
                disabled: false,
            },
        ),
        (
            r#"
#[proc_macro]
pub fn mirror(input: TokenStream) -> TokenStream {
    input
}
"#
                .into(),
            ProcMacro {
                name: Symbol::intern("mirror"),
                kind: ProcMacroKind::Bang,
                expander: sync::Arc::new(MirrorProcMacroExpander),
                disabled: false,
            },
        ),
        (
            r#"
#[proc_macro]
pub fn shorten(input: TokenStream) -> TokenStream {
    loop {}
}
"#
                .into(),
            ProcMacro {
                name: Symbol::intern("shorten"),
                kind: ProcMacroKind::Bang,
                expander: sync::Arc::new(ShortenProcMacroExpander),
                disabled: false,
            },
        ),
        (
            r#"
#[proc_macro_attribute]
pub fn issue_18089(_attr: TokenStream, _item: TokenStream) -> TokenStream {
    loop {}
}
"#
                .into(),
            ProcMacro {
                name: Symbol::intern("issue_18089"),
                kind: ProcMacroKind::Attr,
                expander: sync::Arc::new(Issue18089ProcMacroExpander),
                disabled: false,
            },
        ),
        (
            r#"
#[proc_macro_attribute]
pub fn issue_18840(_attr: TokenStream, _item: TokenStream) -> TokenStream {
    loop {}
}
"#
                .into(),
            ProcMacro {
                name: Symbol::intern("issue_18840"),
                kind: ProcMacroKind::Attr,
                expander: sync::Arc::new(Issue18840ProcMacroExpander),
                disabled: false,
            },
        ),
        (
            r#"
#[proc_macro]
pub fn issue_17479(input: TokenStream) -> TokenStream {
    input
}
"#
                .into(),
            ProcMacro {
                name: Symbol::intern("issue_17479"),
                kind: ProcMacroKind::Bang,
                expander: sync::Arc::new(Issue17479ProcMacroExpander),
                disabled: false,
            },
        ),
        (
            r#"
#[proc_macro_attribute]
pub fn issue_18898(_attr: TokenStream, input: TokenStream) -> TokenStream {
    input
}
"#
                .into(),
            ProcMacro {
                name: Symbol::intern("issue_18898"),
                kind: ProcMacroKind::Bang,
                expander: sync::Arc::new(Issue18898ProcMacroExpander),
                disabled: false,
            },
        ),
        (
            r#"
#[proc_macro_attribute]
pub fn disallow_cfg(_attr: TokenStream, input: TokenStream) -> TokenStream {
    input
}
"#
                .into(),
            ProcMacro {
                name: Symbol::intern("disallow_cfg"),
                kind: ProcMacroKind::Attr,
                expander: sync::Arc::new(DisallowCfgProcMacroExpander),
                disabled: false,
            },
        ),
        (
            r#"
#[proc_macro_attribute]
pub fn generate_suffixed_type(_attr: TokenStream, input: TokenStream) -> TokenStream {
    input
}
"#
                .into(),
            ProcMacro {
                name: Symbol::intern("generate_suffixed_type"),
                kind: ProcMacroKind::Attr,
                expander: sync::Arc::new(GenerateSuffixedTypeProcMacroExpander),
                disabled: false,
            },
        ),
    ])
}
