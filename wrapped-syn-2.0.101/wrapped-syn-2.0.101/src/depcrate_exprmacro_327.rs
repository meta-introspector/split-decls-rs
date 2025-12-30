// Generated macro for macro_327 (macro)
macro_rules! Depcrate_exprmacro_327 {
() => {
// Module: crate::expr
// Provides: {"macro_327"}
// Dependencies: {}
ast_struct ! { # [doc = " A referencing operation: `&a` or `&mut a`."] # [cfg_attr (docsrs , doc (cfg (any (feature = "full" , feature = "derive"))))] pub struct ExprReference { pub attrs : Vec < Attribute >, pub and_token : Token ! [&] , pub mutability : Option < Token ! [mut] >, pub expr : Box < Expr >, } }
};
}
