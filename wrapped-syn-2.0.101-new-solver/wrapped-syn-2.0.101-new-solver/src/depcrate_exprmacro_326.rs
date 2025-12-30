// Generated macro for macro_326 (macro)
macro_rules! Depcrate_exprmacro_326 {
() => {
// Module: crate::expr
// Provides: {"macro_326"}
// Dependencies: {}
ast_struct ! { # [doc = " Address-of operation: `&raw const place` or `&raw mut place`."] # [cfg_attr (docsrs , doc (cfg (feature = "full")))] pub struct ExprRawAddr # full { pub attrs : Vec < Attribute >, pub and_token : Token ! [&] , pub raw : Token ! [raw] , pub mutability : PointerMutability , pub expr : Box < Expr >, } }
};
}
