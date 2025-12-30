// Generated macro for impl_for_int (macro)
macro_rules! Depcrate_to_tokensimpl_for_int {
() => {
// Module: crate::to_tokens
// Provides: {"impl_for_int"}
// Dependencies: {}
macro_rules ! impl_for_int { ($ ($ type : ty => $ method : ident) *) => { $ (impl ToTokenTree for $ type { fn into_token_tree (self) -> TokenTree { TokenTree :: from (Literal ::$ method (self)) } }) * } ; }
};
}
