// Generated macro for impl_70 (impl)
macro_rules! Depcrate_expressionimpl_70 {
() => {
// Module: crate::expression
// Provides: {"impl_70"}
// Dependencies: {}
impl ToTokens for FnCall { fn to_tokens (& self , tokens : & mut TokenStream) { let FnCall (fn_ptr , arguments , turbofish , _requires_unsafe_wrapper) = self ; fn_ptr . to_tokens (tokens) ; if ! turbofish . is_empty () { tokens . append_all (quote ! { ::<# (# turbofish) ,*> }) ; } tokens . append_all (quote ! { (# (# arguments) ,*) }) } }
};
}
