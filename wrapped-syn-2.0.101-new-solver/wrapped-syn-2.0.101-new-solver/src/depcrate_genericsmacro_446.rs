// Generated macro for macro_446 (macro)
macro_rules! Depcrate_genericsmacro_446 {
() => {
// Module: crate::generics
// Provides: {"macro_446"}
// Dependencies: {}
ast_enum ! { # [doc = " A modifier on a trait bound, currently only used for the `?` in"] # [doc = " `?Sized`."] # [cfg_attr (docsrs , doc (cfg (any (feature = "full" , feature = "derive"))))] pub enum TraitBoundModifier { None , Maybe (Token ! [?]) , } }
};
}
