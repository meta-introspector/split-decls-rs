// Generated macro for macro_231 (macro)
macro_rules! Depcrate_tymacro_231 {
() => {
// Module: crate::ty
// Provides: {"macro_231"}
// Dependencies: {}
ast_struct ! { # [doc = " A path like `Foo(A,B) -> C`"] pub struct ParenthesizedParameterData { pub paren_token : tokens :: Paren , # [doc = " `(A, B)`"] pub inputs : Delimited < Ty , tokens :: Comma >, # [doc = " `C`"] pub output : FunctionRetTy , } }
};
}
