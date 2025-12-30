// Generated macro for macro_78 (macro)
macro_rules! Depcrate_genericsmacro_78 {
() => {
// Module: crate::generics
// Provides: {"macro_78"}
// Dependencies: {}
ast_struct ! { # [doc = " A generic type parameter, e.g. `T: Into<String>`."] pub struct TyParam { pub attrs : Vec < Attribute >, pub ident : Ident , pub colon_token : Option < tokens :: Colon >, pub bounds : Delimited < TyParamBound , tokens :: Add >, pub eq_token : Option < tokens :: Eq >, pub default : Option < Ty >, } }
};
}
