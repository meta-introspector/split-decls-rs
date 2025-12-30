// Generated macro for macro_226 (macro)
macro_rules! Depcrate_tymacro_226 {
() => {
// Module: crate::ty
// Provides: {"macro_226"}
// Dependencies: {}
ast_enum ! { # [doc = " Parameters of a path segment."] # [doc = ""] # [doc = " E.g. `<A, B>` as in `Foo<A, B>` or `(A, B)` as in `Foo(A, B)`"] pub enum PathParameters { None , # [doc = " The `<'a, A, B, C>` in `foo::bar::baz::<'a, A, B, C>`"] AngleBracketed (AngleBracketedParameterData) , # [doc = " The `(A, B)` and `C` in `Foo(A, B) -> C`"] Parenthesized (ParenthesizedParameterData) , } }
};
}
