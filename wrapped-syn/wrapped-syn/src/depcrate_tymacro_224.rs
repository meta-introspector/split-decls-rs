// Generated macro for macro_224 (macro)
macro_rules! Depcrate_tymacro_224 {
() => {
// Module: crate::ty
// Provides: {"macro_224"}
// Dependencies: {}
ast_struct ! { # [doc = " A segment of a path: an identifier, an optional lifetime, and a set of types."] # [doc = ""] # [doc = " E.g. `std`, `String` or `Box<T>`"] pub struct PathSegment { # [doc = " The identifier portion of this path segment."] pub ident : Ident , # [doc = " Type/lifetime parameters attached to this path. They come in"] # [doc = " two flavors: `Path<A,B,C>` and `Path(A,B) -> C`. Note that"] # [doc = " this is more than just simple syntactic sugar; the use of"] # [doc = " parens affects the region binding rules, so we preserve the"] # [doc = " distinction."] pub parameters : PathParameters , } }
};
}
