// Generated macro for macro_303 (macro)
macro_rules! Depcrate_exprmacro_303 {
() => {
// Module: crate::expr
// Provides: {"macro_303"}
// Dependencies: {}
ast_struct ! { # [doc = " An async block: `async { ... }`."] # [cfg_attr (docsrs , doc (cfg (feature = "full")))] pub struct ExprAsync # full { pub attrs : Vec < Attribute >, pub async_token : Token ! [async] , pub capture : Option < Token ! [move] >, pub block : Block , } }
};
}
