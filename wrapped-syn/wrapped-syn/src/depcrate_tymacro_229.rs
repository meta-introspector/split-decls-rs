// Generated macro for macro_229 (macro)
macro_rules! Depcrate_tymacro_229 {
() => {
// Module: crate::ty
// Provides: {"macro_229"}
// Dependencies: {}
ast_struct ! { # [doc = " A path like `Foo<'a, T>`"] pub struct AngleBracketedParameterData { pub turbofish : Option < tokens :: Colon2 >, pub lt_token : tokens :: Lt , # [doc = " The lifetime parameters for this path segment."] pub lifetimes : Delimited < Lifetime , tokens :: Comma >, # [doc = " The type parameters for this path segment, if present."] pub types : Delimited < Ty , tokens :: Comma >, # [doc = " Bindings (equality constraints) on associated types, if present."] # [doc = ""] # [doc = " E.g., `Foo<A=Bar>`."] pub bindings : Delimited < TypeBinding , tokens :: Comma >, pub gt_token : tokens :: Gt , } }
};
}
