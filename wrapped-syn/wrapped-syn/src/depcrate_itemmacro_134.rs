// Generated macro for macro_134 (macro)
macro_rules! Depcrate_itemmacro_134 {
() => {
// Module: crate::item
// Provides: {"macro_134"}
// Dependencies: {}
ast_struct ! { # [doc = " Header (not the body) of a function declaration."] # [doc = ""] # [doc = " E.g. `fn foo(bar: baz)`"] pub struct FnDecl { pub fn_token : tokens :: Fn_ , pub paren_token : tokens :: Paren , pub inputs : Delimited < FnArg , tokens :: Comma >, pub output : FunctionRetTy , pub generics : Generics , pub variadic : bool , pub dot_tokens : Option < tokens :: Dot3 >, } }
};
}
