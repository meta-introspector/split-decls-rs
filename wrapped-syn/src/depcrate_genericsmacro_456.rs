// Generated macro for macro_456 (macro)
macro_rules! Depcrate_genericsmacro_456 {
() => {
// Module: crate::generics
// Provides: {"macro_456"}
// Dependencies: {}
ast_enum_of_structs ! { # [doc = " A trait or lifetime used as a bound on a type parameter."] # [cfg_attr (docsrs , doc (cfg (any (feature = "full" , feature = "derive"))))] # [non_exhaustive] pub enum TypeParamBound { Trait (TraitBound) , Lifetime (Lifetime) , PreciseCapture (PreciseCapture) , Verbatim (TokenStream) , } }
};
}
