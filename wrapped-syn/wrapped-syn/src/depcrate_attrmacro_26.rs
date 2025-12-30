// Generated macro for macro_26 (macro)
macro_rules! Depcrate_attrmacro_26 {
() => {
// Module: crate::attr
// Provides: {"macro_26"}
// Dependencies: {}
ast_enum_of_structs ! { # [doc = " Possible values inside of compile-time attribute lists."] # [doc = ""] # [doc = " E.g. the '..' in `#[name(..)]`."] pub enum NestedMetaItem { # [doc = " A full `MetaItem`."] # [doc = ""] # [doc = " E.g. `Copy` in `#[derive(Copy)]` would be a `MetaItem::Term(Ident::from(\"Copy\"))`."] pub MetaItem (MetaItem) , # [doc = " A Rust literal."] # [doc = ""] # [doc = " E.g. `\"name\"` in `#[rename(\"name\")]`."] pub Literal (Lit) , } }
};
}
