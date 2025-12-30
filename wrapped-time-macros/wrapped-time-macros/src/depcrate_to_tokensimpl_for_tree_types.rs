// Generated macro for impl_for_tree_types (macro)
macro_rules! Depcrate_to_tokensimpl_for_tree_types {
() => {
// Module: crate::to_tokens
// Provides: {"impl_for_tree_types"}
// Dependencies: {}
macro_rules ! impl_for_tree_types { ($ ($ type : ty) *) => { $ (impl ToTokenTree for $ type { fn into_token_tree (self) -> TokenTree { TokenTree :: from (self) } }) * } ; }
};
}
