// Generated macro for macro_330 (macro)
macro_rules! Depcrate_exprmacro_330 {
() => {
// Module: crate::expr
// Provides: {"macro_330"}
// Dependencies: {}
ast_struct ! { # [doc = " A struct literal expression: `Point { x: 1, y: 1 }`."] # [doc = ""] # [doc = " The `rest` provides the value of the remaining fields as in `S { a:"] # [doc = " 1, b: 1, ..rest }`."] # [cfg_attr (docsrs , doc (cfg (any (feature = "full" , feature = "derive"))))] pub struct ExprStruct { pub attrs : Vec < Attribute >, pub qself : Option < QSelf >, pub path : Path , pub brace_token : token :: Brace , pub fields : Punctuated < FieldValue , Token ! [,] >, pub dot2_token : Option < Token ! [..] >, pub rest : Option < Box < Expr >>, } }
};
}
