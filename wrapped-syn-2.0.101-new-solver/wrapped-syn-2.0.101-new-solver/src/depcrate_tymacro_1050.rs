// Generated macro for macro_1050 (macro)
macro_rules! Depcrate_tymacro_1050 {
() => {
// Module: crate::ty
// Provides: {"macro_1050"}
// Dependencies: {}
ast_struct ! { # [doc = " The variadic argument of a function pointer like `fn(usize, ...)`."] # [cfg_attr (docsrs , doc (cfg (any (feature = "full" , feature = "derive"))))] pub struct BareVariadic { pub attrs : Vec < Attribute >, pub name : Option < (Ident , Token ! [:]) >, pub dots : Token ! [...] , pub comma : Option < Token ! [,] >, } }
};
}
