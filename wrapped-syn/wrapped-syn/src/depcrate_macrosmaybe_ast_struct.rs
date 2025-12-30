// Generated macro for maybe_ast_struct (macro)
macro_rules! Depcrate_macrosmaybe_ast_struct {
() => {
// Module: crate::macros
// Provides: {"maybe_ast_struct"}
// Dependencies: {}
macro_rules ! maybe_ast_struct { ($ (# [$ attr : meta]) * pub struct $ name : ident) => () ; ($ ($ rest : tt) *) => (ast_struct ! { $ ($ rest) * }) ; }
};
}
