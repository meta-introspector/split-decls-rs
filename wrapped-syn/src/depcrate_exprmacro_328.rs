// Generated macro for macro_328 (macro)
macro_rules! Depcrate_exprmacro_328 {
() => {
// Module: crate::expr
// Provides: {"macro_328"}
// Dependencies: {}
ast_struct ! { # [doc = " Address-of operation: `&raw const place` or `&raw mut place`."] # [cfg_attr (docsrs , doc (cfg (feature = "full")))] pub struct ExprRawAddr # full { pub attrs : Vec < Attribute >, pub and_token : Token ! [&] , pub raw : Token ! [raw] , pub mutability : PointerMutability , pub expr : Box < Expr >, } }
};
}
