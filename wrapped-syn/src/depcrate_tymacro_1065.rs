// Generated macro for macro_1065 (macro)
macro_rules! Depcrate_tymacro_1065 {
() => {
// Module: crate::ty
// Provides: {"macro_1065"}
// Dependencies: {}
ast_struct ! { # [doc = " A tuple type: `(A, B, C, String)`."] # [cfg_attr (docsrs , doc (cfg (any (feature = "full" , feature = "derive"))))] pub struct TypeTuple { pub paren_token : token :: Paren , pub elems : Punctuated < Type , Token ! [,] >, } }
};
}
