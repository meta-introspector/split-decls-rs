// Generated macro for macro_1046 (macro)
macro_rules! Depcrate_tymacro_1046 {
() => {
// Module: crate::ty
// Provides: {"macro_1046"}
// Dependencies: {}
ast_struct ! { # [doc = " A trait object type `dyn Bound1 + Bound2 + Bound3` where `Bound` is a"] # [doc = " trait or a lifetime."] # [cfg_attr (docsrs , doc (cfg (any (feature = "full" , feature = "derive"))))] pub struct TypeTraitObject { pub dyn_token : Option < Token ! [dyn] >, pub bounds : Punctuated < TypeParamBound , Token ! [+] >, } }
};
}
