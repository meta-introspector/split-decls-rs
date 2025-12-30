// Generated macro for macro_807 (macro)
macro_rules! Depcrate_patmacro_807 {
() => {
// Module: crate::pat
// Provides: {"macro_807"}
// Dependencies: {}
ast_struct ! { # [doc = " A struct or struct variant pattern: `Variant { x, y, .. }`."] # [cfg_attr (docsrs , doc (cfg (feature = "full")))] pub struct PatStruct { pub attrs : Vec < Attribute >, pub qself : Option < QSelf >, pub path : Path , pub brace_token : token :: Brace , pub fields : Punctuated < FieldPat , Token ! [,] >, pub rest : Option < PatRest >, } }
};
}
