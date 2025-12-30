// Generated macro for macro_429 (macro)
macro_rules! Depcrate_genericsmacro_429 {
() => {
// Module: crate::generics
// Provides: {"macro_429"}
// Dependencies: {}
ast_struct ! { # [doc = " A const generic parameter: `const LENGTH: usize`."] # [cfg_attr (docsrs , doc (cfg (any (feature = "full" , feature = "derive"))))] pub struct ConstParam { pub attrs : Vec < Attribute >, pub const_token : Token ! [const] , pub ident : Ident , pub colon_token : Token ! [:] , pub ty : Type , pub eq_token : Option < Token ! [=] >, pub default : Option < Expr >, } }
};
}
