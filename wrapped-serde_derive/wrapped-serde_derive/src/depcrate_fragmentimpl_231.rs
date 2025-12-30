// Generated macro for impl_231 (impl)
macro_rules! Depcrate_fragmentimpl_231 {
() => {
// Module: crate::fragment
// Provides: {"impl_231"}
// Dependencies: {}
impl ToTokens for Stmts { fn to_tokens (& self , out : & mut TokenStream) { match & self . 0 { Fragment :: Expr (expr) => expr . to_tokens (out) , Fragment :: Block (block) => block . to_tokens (out) , } } }
};
}
