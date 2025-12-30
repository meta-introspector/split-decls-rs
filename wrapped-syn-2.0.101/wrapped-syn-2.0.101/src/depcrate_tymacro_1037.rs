// Generated macro for macro_1037 (macro)
macro_rules! Depcrate_tymacro_1037 {
() => {
// Module: crate::ty
// Provides: {"macro_1037"}
// Dependencies: {}
ast_struct ! { # [doc = " An `impl Bound1 + Bound2 + Bound3` type where `Bound` is a trait or"] # [doc = " a lifetime."] # [cfg_attr (docsrs , doc (cfg (any (feature = "full" , feature = "derive"))))] pub struct TypeImplTrait { pub impl_token : Token ! [impl] , pub bounds : Punctuated < TypeParamBound , Token ! [+] >, } }
};
}
