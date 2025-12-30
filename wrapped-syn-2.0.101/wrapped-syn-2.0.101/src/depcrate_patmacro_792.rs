// Generated macro for macro_792 (macro)
macro_rules! Depcrate_patmacro_792 {
() => {
// Module: crate::pat
// Provides: {"macro_792"}
// Dependencies: {}
ast_struct ! { # [doc = " A tuple pattern: `(a, b)`."] # [cfg_attr (docsrs , doc (cfg (feature = "full")))] pub struct PatTuple { pub attrs : Vec < Attribute >, pub paren_token : token :: Paren , pub elems : Punctuated < Pat , Token ! [,] >, } }
};
}
