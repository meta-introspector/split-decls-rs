// Generated macro for impl_233 (impl)
macro_rules! Depcrate_fragmentimpl_233 {
() => {
// Module: crate::fragment
// Provides: {"impl_233"}
// Dependencies: {}
impl ToTokens for Match { fn to_tokens (& self , out : & mut TokenStream) { match & self . 0 { Fragment :: Expr (expr) => { expr . to_tokens (out) ; < Token ! [,] > :: default () . to_tokens (out) ; } Fragment :: Block (block) => { token :: Brace :: default () . surround (out , | out | block . to_tokens (out)) ; } } } }
};
}
