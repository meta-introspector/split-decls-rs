// Generated macro for impl_25 (impl)
macro_rules! Depcrate_attrimpl_25 {
() => {
// Module: crate::attr
// Provides: {"impl_25"}
// Dependencies: {}
impl MetaItem { # [doc = " Name of the item."] # [doc = ""] # [doc = " E.g. `test` as in `#[test]`, `derive` as in `#[derive(..)]`, and"] # [doc = " `feature` as in `#[feature = \"foo\"]`."] pub fn name (& self) -> & str { match * self { MetaItem :: Term (ref name) => name . as_ref () , MetaItem :: NameValue (ref pair) => pair . ident . as_ref () , MetaItem :: List (ref list) => list . ident . as_ref () , } } }
};
}
