// Generated macro for macro_234 (macro)
macro_rules! Depcrate_tymacro_234 {
() => {
// Module: crate::ty
// Provides: {"macro_234"}
// Dependencies: {}
ast_struct ! { pub struct BareFnTy { pub lifetimes : Option < BoundLifetimes >, pub unsafety : Unsafety , pub abi : Option < Abi >, pub fn_token : tokens :: Fn_ , pub paren_token : tokens :: Paren , pub inputs : Delimited < BareFnArg , tokens :: Comma >, pub variadic : Option < tokens :: Dot3 >, pub output : FunctionRetTy , } }
};
}
