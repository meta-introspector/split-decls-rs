// Generated macro for private (module)
macro_rules! Depcrate_tokenprivate {
() => {
// Module: crate::token
// Provides: {"private"}
// Dependencies: {}
pub (crate) mod private { # [cfg (feature = "parsing")] use crate :: buffer :: Cursor ; use proc_macro2 :: Span ; # [cfg (feature = "parsing")] pub trait Sealed { } # [doc = " Support writing `token.span` rather than `token.spans[0]` on tokens that"] # [doc = " hold a single span."] # [repr (transparent)] # [allow (unknown_lints , repr_transparent_external_private_fields)] pub struct WithSpan { pub span : Span , } # [doc (hidden)] # [cfg (feature = "parsing")] pub trait CustomToken { fn peek (cursor : Cursor) -> bool ; fn display () -> & 'static str ; } }
};
}
