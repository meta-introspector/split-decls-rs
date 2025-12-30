// Generated macro for macro_240 (macro)
macro_rules! Depcrate_tymacro_240 {
() => {
// Module: crate::ty
// Provides: {"macro_240"}
// Dependencies: {}
ast_enum ! { pub enum FunctionRetTy { # [doc = " Return type is not specified."] # [doc = ""] # [doc = " Functions default to `()` and"] # [doc = " closures default to inference. Span points to where return"] # [doc = " type would be inserted."] Default , # [doc = " Everything else"] Ty (Ty , tokens :: RArrow) , } }
};
}
