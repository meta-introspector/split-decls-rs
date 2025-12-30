// Generated macro for macro_793 (macro)
macro_rules! Depcrate_patmacro_793 {
() => {
// Module: crate::pat
// Provides: {"macro_793"}
// Dependencies: {}
ast_struct ! { # [doc = " A tuple struct or tuple variant pattern: `Variant(x, y, .., z)`."] # [cfg_attr (docsrs , doc (cfg (feature = "full")))] pub struct PatTupleStruct { pub attrs : Vec < Attribute >, pub qself : Option < QSelf >, pub path : Path , pub paren_token : token :: Paren , pub elems : Punctuated < Pat , Token ! [,] >, } }
};
}
