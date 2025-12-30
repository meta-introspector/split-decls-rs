// Generated macro for macro_24 (macro)
macro_rules! Depcrate_attrmacro_24 {
() => {
// Module: crate::attr
// Provides: {"macro_24"}
// Dependencies: {}
ast_enum_of_structs ! { # [doc = " A compile-time attribute item."] # [doc = ""] # [doc = " E.g. `#[test]`, `#[derive(..)]` or `#[feature = \"foo\"]`"] pub enum MetaItem { # [doc = " Term meta item."] # [doc = ""] # [doc = " E.g. `test` as in `#[test]`"] pub Term (Ident) , # [doc = " List meta item."] # [doc = ""] # [doc = " E.g. `derive(..)` as in `#[derive(..)]`"] pub List (MetaItemList { # [doc = " Name of this attribute."] # [doc = ""] # [doc = " E.g. `derive` in `#[derive(..)]`"] pub ident : Ident , pub paren_token : tokens :: Paren , # [doc = " Arguments to this attribute"] # [doc = ""] # [doc = " E.g. `..` in `#[derive(..)]`"] pub nested : Delimited < NestedMetaItem , tokens :: Comma >, }) , # [doc = " Name-value meta item."] # [doc = ""] # [doc = " E.g. `feature = \"foo\"` as in `#[feature = \"foo\"]`"] pub NameValue (MetaNameValue { # [doc = " Name of this attribute."] # [doc = ""] # [doc = " E.g. `feature` in `#[feature = \"foo\"]`"] pub ident : Ident , pub eq_token : tokens :: Eq , # [doc = " Arguments to this attribute"] # [doc = ""] # [doc = " E.g. `\"foo\"` in `#[feature = \"foo\"]`"] pub lit : Lit , }) , } }
};
}
