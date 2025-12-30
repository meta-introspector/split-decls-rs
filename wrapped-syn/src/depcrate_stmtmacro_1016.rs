// Generated macro for macro_1016 (macro)
macro_rules! Depcrate_stmtmacro_1016 {
() => {
// Module: crate::stmt
// Provides: {"macro_1016"}
// Dependencies: {}
ast_struct ! { # [doc = " A macro invocation in statement position."] # [doc = ""] # [doc = " Syntactically it's ambiguous which other kind of statement this macro"] # [doc = " would expand to. It can be any of local variable (`let`), item, or"] # [doc = " expression."] # [cfg_attr (docsrs , doc (cfg (feature = "full")))] pub struct StmtMacro { pub attrs : Vec < Attribute >, pub mac : Macro , pub semi_token : Option < Token ! [;] >, } }
};
}
