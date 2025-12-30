// Generated macro for impl_229 (impl)
macro_rules! Depcrate_fragmentimpl_229 {
() => {
// Module: crate::fragment
// Provides: {"impl_229"}
// Dependencies: {}
impl ToTokens for Expr { fn to_tokens (& self , out : & mut TokenStream) { match & self . 0 { Fragment :: Expr (expr) => expr . to_tokens (out) , Fragment :: Block (block) => { token :: Brace :: default () . surround (out , | out | block . to_tokens (out)) ; } } } }
};
}
